//! Types for the `m.beacon.start` event.

use std::ops::Deref;

use js_int::{uint, UInt};
use ruma_common::{serde::StringEnum, MilliSecondsSinceUnixEpoch};
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use crate::PrivOwnedStr;

mod beacon_answers_serde;

use beacon_answers_serde::BeaconAnswersDeHelper;

use super::{
    compile_beacon_results,
    end::{BeaconEndEventContent, BeaconResultsContentBlock},
    generate_beacon_end_fallback_text, BeaconResponseData,
};
use crate::{message::TextContentBlock, room::message::Relation};

/// The payload for a beacon start event.
///
/// This is the event content that should be sent for room versions that support extensible events.
/// As of Matrix 1.7, none of the stable room versions (1 through 10) support extensible events.
///
/// To send a beacon start event for a room version that does not support extensible events, use
/// [`UnstableBeaconStartEventContent`].
///
/// [`UnstableBeaconStartEventContent`]: super::unstable_start::UnstableBeaconStartEventContent
#[derive(Clone, Debug, Serialize, Deserialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.beacon.start", kind = MessageLike, without_relation)]
pub struct BeaconStartEventContent {
    /// The beacon content of the message.
    #[serde(rename = "m.beacon")]
    pub beacon: BeaconContentBlock,

    /// Text representation of the message, for clients that don't support beacons.
    #[serde(rename = "m.text")]
    pub text: TextContentBlock,

    /// Information about related messages.
    #[serde(
        flatten,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::room::message::relation_serde::deserialize_relation"
    )]
    pub relates_to: Option<Relation<BeaconStartEventContentWithoutRelation>>,

    /// Whether this message is automated.
    #[cfg(feature = "unstable-msc3955")]
    #[serde(
        default,
        skip_serializing_if = "ruma_common::serde::is_default",
        rename = "org.matrix.msc1767.automated"
    )]
    pub automated: bool,
}

impl BeaconStartEventContent {
    /// Creates a new `BeaconStartEventContent` with the given fallback representation and beacon
    /// content.
    pub fn new(text: TextContentBlock, beacon: BeaconContentBlock) -> Self {
        Self {
            beacon,
            text,
            relates_to: None,
            #[cfg(feature = "unstable-msc3955")]
            automated: false,
        }
    }

    /// Creates a new `BeaconStartEventContent` with the given plain text fallback
    /// representation and beacon content.
    pub fn with_plain_text(plain_text: impl Into<String>, beacon: BeaconContentBlock) -> Self {
        Self::new(TextContentBlock::plain(plain_text), beacon)
    }
}

impl OriginalSyncBeaconStartEvent {
    /// Compile the results for this beacon with the given response into a `BeaconEndEventContent`.
    ///
    /// It generates a default text representation of the results in English.
    ///
    /// This uses [`compile_beacon_results()`] internally.
    pub fn compile_results<'a>(
        &'a self,
        responses: impl IntoIterator<Item = BeaconResponseData<'a>>,
    ) -> BeaconEndEventContent {
        let full_results = compile_beacon_results(
            &self.content.beacon,
            responses,
            Some(MilliSecondsSinceUnixEpoch::now()),
        );
        let results =
            full_results.into_iter().map(|(id, users)| (id, users.len())).collect::<Vec<_>>();

        // Construct the results and get the top answer(s).
        let beacon_results = BeaconResultsContentBlock::from_iter(
            results
                .iter()
                .map(|(id, count)| ((*id).to_owned(), (*count).try_into().unwrap_or(UInt::MAX))),
        );

        // Get the text representation of the best answers.
        let answers = self
            .content
            .beacon
            .answers
            .iter()
            .map(|a| {
                let text = a.text.find_plain().unwrap_or(&a.id);
                (a.id.as_str(), text)
            })
            .collect::<Vec<_>>();
        let plain_text = generate_beacon_end_fallback_text(&answers, results.into_iter());

        let mut end = BeaconEndEventContent::with_plain_text(plain_text, self.event_id.clone());
        end.beacon_results = Some(beacon_results);

        end
    }
}

/// A block for beacon content.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct BeaconContentBlock {
    /// The question of the beacon.
    pub question: BeaconQuestion,

    /// The kind of the beacon.
    #[serde(default, skip_serializing_if = "ruma_common::serde::is_default")]
    pub kind: BeaconKind,

    /// The maximum number of responses a user is able to select.
    ///
    /// Must be greater or equal to `1`.
    ///
    /// Defaults to `1`.
    #[serde(
        default = "BeaconContentBlock::default_max_selections",
        skip_serializing_if = "BeaconContentBlock::max_selections_is_default"
    )]
    pub max_selections: UInt,

    /// The possible answers to the beacon.
    pub answers: BeaconAnswers,
}

impl BeaconContentBlock {
    /// Creates a new `BeaconStartContent` with the given question and answers.
    pub fn new(question: TextContentBlock, answers: BeaconAnswers) -> Self {
        Self {
            question: question.into(),
            kind: Default::default(),
            max_selections: Self::default_max_selections(),
            answers,
        }
    }

    pub(super) fn default_max_selections() -> UInt {
        uint!(1)
    }

    fn max_selections_is_default(max_selections: &UInt) -> bool {
        max_selections == &Self::default_max_selections()
    }
}

/// The question of a beacon.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct BeaconQuestion {
    /// The text representation of the question.
    #[serde(rename = "m.text")]
    pub text: TextContentBlock,
}

impl From<TextContentBlock> for BeaconQuestion {
    fn from(text: TextContentBlock) -> Self {
        Self { text }
    }
}

/// The kind of beacon.
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/doc/string_enum.md"))]
#[derive(Clone, Default, PartialEq, Eq, StringEnum)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub enum BeaconKind {
    /// The results are revealed once the beacon is closed.
    #[default]
    #[ruma_enum(rename = "m.undisclosed")]
    Undisclosed,

    /// The votes are visible up until and including when the beacon is closed.
    #[ruma_enum(rename = "m.disclosed")]
    Disclosed,

    #[doc(hidden)]
    _Custom(PrivOwnedStr),
}

/// The answers to a beacon.
///
/// Must include between 1 and 20 `BeaconAnswer`s.
///
/// To build this, use the `TryFrom` implementations.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "BeaconAnswersDeHelper")]
pub struct BeaconAnswers(Vec<BeaconAnswer>);

impl BeaconAnswers {
    /// The smallest number of values contained in a `BeaconAnswers`.
    pub const MIN_LENGTH: usize = 1;

    /// The largest number of values contained in a `BeaconAnswers`.
    pub const MAX_LENGTH: usize = 20;
}

/// An error encountered when trying to convert to a `BeaconAnswers`.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum BeaconAnswersError {
    /// There are more than [`BeaconAnswers::MAX_LENGTH`] values.
    #[error("too many values")]
    TooManyValues,
    /// There are less that [`BeaconAnswers::MIN_LENGTH`] values.
    #[error("not enough values")]
    NotEnoughValues,
}

impl TryFrom<Vec<BeaconAnswer>> for BeaconAnswers {
    type Error = BeaconAnswersError;

    fn try_from(value: Vec<BeaconAnswer>) -> Result<Self, Self::Error> {
        if value.len() < Self::MIN_LENGTH {
            Err(BeaconAnswersError::NotEnoughValues)
        } else if value.len() > Self::MAX_LENGTH {
            Err(BeaconAnswersError::TooManyValues)
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<&[BeaconAnswer]> for BeaconAnswers {
    type Error = BeaconAnswersError;

    fn try_from(value: &[BeaconAnswer]) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl Deref for BeaconAnswers {
    type Target = [BeaconAnswer];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Beacon answer.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct BeaconAnswer {
    /// The ID of the answer.
    ///
    /// This must be unique among the answers of a beacon.
    #[serde(rename = "m.id")]
    pub id: String,

    /// The text representation of the answer.
    #[serde(rename = "m.text")]
    pub text: TextContentBlock,
}

impl BeaconAnswer {
    /// Creates a new `BeaconAnswer` with the given id and text representation.
    pub fn new(id: String, text: TextContentBlock) -> Self {
        Self { id, text }
    }
}
