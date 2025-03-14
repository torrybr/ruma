#![allow(clippy::disallowed_types)]

use std::{collections::HashMap, path::PathBuf};

#[cfg(feature = "default")]
use reqwest::blocking::Client;
use semver::Version;
use serde::{de::IgnoredAny, Deserialize};
#[cfg(feature = "default")]
use toml_edit::{value, DocumentMut};
#[cfg(feature = "default")]
use xshell::Shell;

#[cfg(feature = "default")]
use crate::cmd;
use crate::{util::ask_yes_no, Metadata, Result};

const CRATESIO_API: &str = "https://crates.io/api/v1/crates";

/// A cargo package.
#[derive(Clone, Debug, Deserialize)]
pub struct Package {
    /// The package name
    pub name: String,

    /// The package version.
    pub version: Version,

    /// The package's manifest path.
    pub manifest_path: PathBuf,

    /// A list of the package dependencies.
    #[serde(default)]
    pub dependencies: Vec<Dependency>,

    /// A map of the package features.
    #[serde(default)]
    pub features: HashMap<String, Vec<String>>,
}

impl Package {
    /// Whether this package has a way to enable the given feature from the given package.
    pub fn can_enable_feature(&self, package_name: &str, feature_name: &str) -> bool {
        for activated_feature in self.features.values().flatten() {
            // Remove optional `dep:` at the start.
            let remaining = activated_feature.trim_start_matches("dep:");

            // Check that we have the package name.
            let Some(remaining) = remaining.strip_prefix(package_name) else {
                continue;
            };

            if remaining.is_empty() {
                // The feature only enables the dependency.
                continue;
            }

            // Remove optional `?`.
            let remaining = remaining.trim_start_matches('?');

            let Some(remaining) = remaining.strip_prefix('/') else {
                // This is another package name starting with the same string.
                continue;
            };

            // Finally, only the feature name is remaining.
            if remaining == feature_name {
                return true;
            }
        }

        false
    }
}

#[cfg(feature = "default")]
impl Package {
    /// Update the version of this crate.
    pub fn update_version(&mut self, sh: &Shell, version: &Version, dry_run: bool) -> Result<()> {
        println!("Updating {} to version {version}…", self.name);

        if !dry_run {
            let mut document = sh.read_file(&self.manifest_path)?.parse::<DocumentMut>()?;

            document["package"]["version"] = value(version.to_string());

            sh.write_file(&self.manifest_path, document.to_string())?;
        }

        self.version = version.clone();

        Ok(())
    }

    /// Update the version of this crate in dependant crates' manifests, with the given version
    /// prefix.
    pub(crate) fn update_dependants(
        &self,
        sh: &Shell,
        metadata: &Metadata,
        dry_run: bool,
    ) -> Result<()> {
        if self.name == "ruma" {
            for package in metadata.packages.iter().filter(|p| {
                p.manifest_path.starts_with(&metadata.workspace_root)
                    && p.dependencies.iter().any(|d| d.name == self.name)
            }) {
                println!("Updating dependency in {} crate…", package.name);

                if !dry_run {
                    let mut document =
                        sh.read_file(&package.manifest_path)?.parse::<DocumentMut>()?;

                    let version = if !self.version.pre.is_empty() {
                        format!("={}", self.version)
                    } else {
                        self.version.to_string()
                    };

                    for dependency in package.dependencies.iter().filter(|d| d.name == self.name) {
                        let kind = match dependency.kind {
                            Some(DependencyKind::Dev) => "dev-dependencies",
                            Some(DependencyKind::Build) => "build-dependencies",
                            None => "dependencies",
                        };

                        document[kind][&self.name]["version"] = value(version.as_str());
                    }

                    sh.write_file(&package.manifest_path, document.to_string())?;
                }
            }
        } else {
            let workspace_manifest_path = metadata.workspace_root.join("Cargo.toml");
            let mut document = sh.read_file(&workspace_manifest_path)?.parse::<DocumentMut>()?;
            let workspace_deps = &mut document["workspace"]["dependencies"];

            println!("Updating workspace dependency…");
            assert!(workspace_deps.get(&self.name).is_some());

            if !dry_run {
                let version = if self.name == "ruma-macros" || !self.version.pre.is_empty() {
                    format!("={}", self.version)
                } else {
                    self.version.to_string()
                };

                workspace_deps[&self.name]["version"] = value(version.as_str());

                sh.write_file(&workspace_manifest_path, document.to_string())?;
            }
        }

        Ok(())
    }

    /// Update the changelog for the release of the current version, if needed.
    pub fn update_changelog(&self, sh: &Shell) -> Result<()> {
        self.changes_inner(sh, true)?;
        Ok(())
    }

    /// Get the changes for the current version.
    pub fn changes(&self, sh: &Shell) -> Result<String> {
        self.changes_inner(sh, false)
    }

    /// Get the changes for the current version.
    ///
    /// If `update` is `true`, the changelog is updated if needed.
    fn changes_inner(&self, sh: &Shell, update: bool) -> Result<String> {
        if self.name == "ruma-macros" {
            // ruma-macros doesn't have a changelog and won't create a tag.
            return Ok(String::new());
        }

        let mut changelog_path = self.manifest_path.clone();
        changelog_path.set_file_name("CHANGELOG.md");

        let changelog = sh.read_file(&changelog_path)?;
        let version = Version {
            pre: semver::Prerelease::EMPTY,
            build: semver::BuildMetadata::EMPTY,
            ..self.version.clone()
        };

        let (update, title_start) = if let Some(pos) = changelog.find(&format!("# {version}\n")) {
            (false, pos)
        } else if update
            && (changelog.starts_with(&format!("# {version} (unreleased)\n"))
                || changelog.starts_with("# [unreleased]\n"))
        {
            (true, 0)
        } else {
            return Err("Could not find version title in changelog".into());
        };

        let changes_start = match changelog[title_start..].find('\n') {
            Some(p) => title_start + p + 1,
            None => {
                return Err("Could not find end of version title in changelog".into());
            }
        };

        let changes_end = match changelog[changes_start..].find("\n# ") {
            Some(p) => changes_start + p,
            None => changelog.len(),
        };

        let changes = match changelog[changes_start..changes_end].trim() {
            "" => "No changes for this version",
            s => s,
        };

        if update {
            let rest = &changelog[changes_end..];
            let changelog = format!("# [unreleased]\n\n# {}\n\n{changes}\n{rest}", self.version);

            sh.write_file(&changelog_path, changelog)?;
        }

        Ok(changes.to_owned())
    }

    /// Check if the current version of the crate is published on crates.io.
    pub fn is_published(&self, client: &Client) -> Result<bool> {
        let response: CratesIoCrate =
            client.get(format!("{CRATESIO_API}/{}/{}", self.name, self.version)).send()?.json()?;

        Ok(response.version.is_some())
    }

    /// Publish this package on crates.io.
    pub fn publish(&self, sh: &Shell, client: &Client, dry_run: bool) -> Result<()> {
        println!("Publishing {} {} on crates.io…", self.name, self.version);
        let _dir = sh.push_dir(self.manifest_path.parent().unwrap());

        if self.is_published(client)? {
            if !ask_yes_no("This version is already published. Skip this step and continue?")? {
                return Err("Release interrupted by user.".into());
            }
        } else if !dry_run {
            cmd!(sh, "cargo publish").run()?;
        }

        Ok(())
    }
}

/// A cargo package dependency.
#[derive(Clone, Debug, Deserialize)]
pub struct Dependency {
    /// The package name.
    pub name: String,

    /// The kind of the dependency.
    pub kind: Option<DependencyKind>,
}

/// The kind of a cargo package dependency.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DependencyKind {
    /// A dev dependency.
    Dev,

    /// A build dependency.
    Build,
}

#[cfg(feature = "default")]
/// A crate from the `GET /crates/{crate}` endpoint of crates.io.
#[derive(Deserialize)]
struct CratesIoCrate {
    version: Option<IgnoredAny>,
}
