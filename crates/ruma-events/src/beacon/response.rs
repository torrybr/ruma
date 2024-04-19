//! Types for the `m.beacon.response` event.

use std::{ops::Deref, vec};

use ruma_common::OwnedEventId;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use super::{start::BeaconContentBlock, validate_selections, BeaconResponseData};
use crate::relation::Reference;

/// The payload for a beacon response event.
///
/// This is the event content that should be sent for room versions that support extensible events.
/// As of Matrix 1.7, none of the stable room versions (1 through 10) support extensible events.
///
/// To send a beacon response event for a room version that does not support extensible events, use
/// [`UnstableBeaconResponseEventContent`].
///
/// [`UnstableBeaconResponseEventContent`]: super::unstable_response::UnstableBeaconResponseEventContent
#[derive(Clone, Debug, Serialize, Deserialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.beacon.response", kind = MessageLike)]
pub struct BeaconResponseEventContent {
    /// The user's selection.
    #[serde(rename = "m.selections")]
    pub selections: SelectionsContentBlock,

    /// Whether this message is automated.
    #[cfg(feature = "unstable-msc3955")]
    #[serde(
        default,
        skip_serializing_if = "ruma_common::serde::is_default",
        rename = "org.matrix.msc1767.automated"
    )]
    pub automated: bool,

    /// Information about the beacon start event this responds to.
    #[serde(rename = "m.relates_to")]
    pub relates_to: Reference,
}

impl BeaconResponseEventContent {
    /// Creates a new `BeaconResponseEventContent` that responds to the given beacon start event ID,
    /// with the given beacon response content.
    pub fn new(selections: SelectionsContentBlock, beacon_start_id: OwnedEventId) -> Self {
        Self {
            selections,
            #[cfg(feature = "unstable-msc3955")]
            automated: false,
            relates_to: Reference::new(beacon_start_id),
        }
    }
}

impl OriginalSyncBeaconResponseEvent {
    /// Get the data from this response necessary to compile beacon results.
    pub fn data(&self) -> BeaconResponseData<'_> {
        BeaconResponseData {
            sender: &self.sender,
            origin_server_ts: self.origin_server_ts,
            selections: &self.content.selections,
        }
    }
}

impl OriginalBeaconResponseEvent {
    /// Get the data from this response necessary to compile beacon results.
    pub fn data(&self) -> BeaconResponseData<'_> {
        BeaconResponseData {
            sender: &self.sender,
            origin_server_ts: self.origin_server_ts,
            selections: &self.content.selections,
        }
    }
}

/// A block for selections content.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct SelectionsContentBlock(Vec<String>);

impl SelectionsContentBlock {
    /// Whether this `SelectionsContentBlock` is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Validate these selections against the given `BeaconContentBlock`.
    ///
    /// Returns the list of valid selections in this `SelectionsContentBlock`, or `None` if there is
    /// no valid selection.
    pub fn validate<'a>(
        &'a self,
        beacon: &BeaconContentBlock,
    ) -> Option<impl Iterator<Item = &'a str>> {
        let answer_ids = beacon.answers.iter().map(|a| a.id.as_str()).collect();
        validate_selections(&answer_ids, beacon.max_selections, &self.0)
    }
}

impl From<Vec<String>> for SelectionsContentBlock {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

impl IntoIterator for SelectionsContentBlock {
    type Item = String;
    type IntoIter = vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<String> for SelectionsContentBlock {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl Deref for SelectionsContentBlock {
    type Target = [String];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
