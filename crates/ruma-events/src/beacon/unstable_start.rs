//! Types for the `org.matrix.msc3489.beacon.start` event, the unstable version of `m.beacon.start`.

use std::ops::Deref;

use js_int::UInt;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

mod content_serde;
mod unstable_beacon_answers_serde;
mod unstable_beacon_kind_serde;

use ruma_common::{MilliSecondsSinceUnixEpoch, OwnedEventId};

use self::unstable_beacon_answers_serde::UnstableBeaconAnswersDeHelper;
use super::{
    compile_unstable_beacon_results, generate_beacon_end_fallback_text,
    start::{BeaconAnswers, BeaconAnswersError, BeaconContentBlock, BeaconKind},
    unstable_end::UnstableBeaconEndEventContent,
    BeaconResponseData,
};
use crate::{
    relation::Replacement, room::message::RelationWithoutReplacement, EventContent,
    MessageLikeEventContent, MessageLikeEventType, RedactContent, RedactedMessageLikeEventContent,
    StaticEventContent,
};

/// The payload for an unstable beacon start event.
///
/// This is the event content that should be sent for room versions that don't support extensible
/// events. As of Matrix 1.7, none of the stable room versions (1 through 10) support extensible
/// events.
///
/// To send a beacon start event for a room version that supports extensible events, use
/// [`BeaconStartEventContent`].
///
/// [`BeaconStartEventContent`]: super::start::BeaconStartEventContent
#[derive(Clone, Debug, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "org.matrix.msc3489.beacon.start", kind = MessageLike, custom_redacted)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum UnstableBeaconStartEventContent {
    /// A new beacon start event.
    New(NewUnstableBeaconStartEventContent),

    /// A replacement beacon start event.
    Replacement(ReplacementUnstableBeaconStartEventContent),
}

impl UnstableBeaconStartEventContent {
    /// Get the beacon start content of this event content.
    pub fn beacon_start(&self) -> &UnstableBeaconStartContentBlock {
        match self {
            Self::New(c) => &c.beacon_start,
            Self::Replacement(c) => &c.relates_to.new_content.beacon_start,
        }
    }
}

impl RedactContent for UnstableBeaconStartEventContent {
    type Redacted = RedactedUnstableBeaconStartEventContent;

    fn redact(self, _version: &crate::RoomVersionId) -> Self::Redacted {
        RedactedUnstableBeaconStartEventContent::default()
    }
}

impl From<NewUnstableBeaconStartEventContent> for UnstableBeaconStartEventContent {
    fn from(value: NewUnstableBeaconStartEventContent) -> Self {
        Self::New(value)
    }
}

impl From<ReplacementUnstableBeaconStartEventContent> for UnstableBeaconStartEventContent {
    fn from(value: ReplacementUnstableBeaconStartEventContent) -> Self {
        Self::Replacement(value)
    }
}

impl OriginalSyncUnstableBeaconStartEvent {
    /// Compile the results for this beacon with the given response into an
    /// `UnstableBeaconEndEventContent`.
    ///
    /// It generates a default text representation of the results in English.
    ///
    /// This uses [`compile_unstable_beacon_results()`] internally.
    pub fn compile_results<'a>(
        &'a self,
        responses: impl IntoIterator<Item = BeaconResponseData<'a>>,
    ) -> UnstableBeaconEndEventContent {
        let beacon_start = self.content.beacon_start();

        let full_results = compile_unstable_beacon_results(
            beacon_start,
            responses,
            Some(MilliSecondsSinceUnixEpoch::now()),
        );
        let results =
            full_results.into_iter().map(|(id, users)| (id, users.len())).collect::<Vec<_>>();

        // Get the text representation of the best answers.
        let answers = beacon_start
            .answers
            .iter()
            .map(|a| (a.id.as_str(), a.text.as_str()))
            .collect::<Vec<_>>();
        let plain_text = generate_beacon_end_fallback_text(&answers, results.into_iter());

        UnstableBeaconEndEventContent::new(plain_text, self.event_id.clone())
    }
}

/// A new unstable beacon start event.
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct NewUnstableBeaconStartEventContent {
    /// The beacon content of the message.
    #[serde(rename = "org.matrix.msc3489.beacon.start")]
    pub beacon_start: UnstableBeaconStartContentBlock,

    /// Text representation of the message, for clients that don't support beacons.
    #[serde(rename = "org.matrix.msc1767.text")]
    pub text: Option<String>,

    /// Information about related messages.
    #[serde(rename = "m.relates_to", skip_serializing_if = "Option::is_none")]
    pub relates_to: Option<RelationWithoutReplacement>,
}

impl NewUnstableBeaconStartEventContent {
    /// Creates a `NewUnstableBeaconStartEventContent` with the given beacon content.
    pub fn new(beacon_start: UnstableBeaconStartContentBlock) -> Self {
        Self { beacon_start, text: None, relates_to: None }
    }

    /// Creates a `NewUnstableBeaconStartEventContent` with the given plain text fallback
    /// representation and beacon content.
    pub fn plain_text(
        text: impl Into<String>,
        beacon_start: UnstableBeaconStartContentBlock,
    ) -> Self {
        Self { beacon_start, text: Some(text.into()), relates_to: None }
    }
}

impl EventContent for NewUnstableBeaconStartEventContent {
    type EventType = MessageLikeEventType;

    fn event_type(&self) -> Self::EventType {
        MessageLikeEventType::UnstableBeaconStart
    }
}

impl StaticEventContent for NewUnstableBeaconStartEventContent {
    const TYPE: &'static str = "org.matrix.msc3489.beacon.start";
}

impl MessageLikeEventContent for NewUnstableBeaconStartEventContent {}

/// Form of [`NewUnstableBeaconStartEventContent`] without relation.
///
/// To construct this type, construct a [`NewUnstableBeaconStartEventContent`] and then use one of
/// its `::from()` / `.into()` methods.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct NewUnstableBeaconStartEventContentWithoutRelation {
    /// The beacon content of the message.
    #[serde(rename = "org.matrix.msc3489.beacon.start")]
    pub beacon_start: UnstableBeaconStartContentBlock,

    /// Text representation of the message, for clients that don't support beacons.
    #[serde(rename = "org.matrix.msc1767.text")]
    pub text: Option<String>,
}

impl From<NewUnstableBeaconStartEventContent>
    for NewUnstableBeaconStartEventContentWithoutRelation
{
    fn from(value: NewUnstableBeaconStartEventContent) -> Self {
        let NewUnstableBeaconStartEventContent { beacon_start, text, .. } = value;
        Self { beacon_start, text }
    }
}

/// A replacement unstable beacon start event.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct ReplacementUnstableBeaconStartEventContent {
    /// The beacon content of the message.
    pub beacon_start: Option<UnstableBeaconStartContentBlock>,

    /// Text representation of the message, for clients that don't support beacons.
    pub text: Option<String>,

    /// Information about related messages.
    pub relates_to: Replacement<NewUnstableBeaconStartEventContentWithoutRelation>,
}

impl ReplacementUnstableBeaconStartEventContent {
    /// Creates a `ReplacementUnstableBeaconStartEventContent` with the given beacon content that
    /// replaces the event with the given ID.
    ///
    /// The constructed content does not have a fallback by default.
    pub fn new(beacon_start: UnstableBeaconStartContentBlock, replaces: OwnedEventId) -> Self {
        Self {
            beacon_start: None,
            text: None,
            relates_to: Replacement {
                event_id: replaces,
                new_content: NewUnstableBeaconStartEventContent::new(beacon_start).into(),
            },
        }
    }

    /// Creates a `ReplacementUnstableBeaconStartEventContent` with the given plain text fallback
    /// representation and beacon content that replaces the event with the given ID.
    ///
    /// The constructed content does not have a fallback by default.
    pub fn plain_text(
        text: impl Into<String>,
        beacon_start: UnstableBeaconStartContentBlock,
        replaces: OwnedEventId,
    ) -> Self {
        Self {
            beacon_start: None,
            text: None,
            relates_to: Replacement {
                event_id: replaces,
                new_content: NewUnstableBeaconStartEventContent::plain_text(text, beacon_start)
                    .into(),
            },
        }
    }
}

impl EventContent for ReplacementUnstableBeaconStartEventContent {
    type EventType = MessageLikeEventType;

    fn event_type(&self) -> Self::EventType {
        MessageLikeEventType::UnstableBeaconStart
    }
}

impl StaticEventContent for ReplacementUnstableBeaconStartEventContent {
    const TYPE: &'static str = "org.matrix.msc3489.beacon.start";
}

impl MessageLikeEventContent for ReplacementUnstableBeaconStartEventContent {}

/// Redacted form of UnstableBeaconStartEventContent
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct RedactedUnstableBeaconStartEventContent {}

impl RedactedUnstableBeaconStartEventContent {
    /// Creates an empty RedactedUnstableBeaconStartEventContent.
    pub fn new() -> RedactedUnstableBeaconStartEventContent {
        Self::default()
    }
}

impl EventContent for RedactedUnstableBeaconStartEventContent {
    type EventType = MessageLikeEventType;

    fn event_type(&self) -> Self::EventType {
        MessageLikeEventType::UnstableBeaconStart
    }
}

impl StaticEventContent for RedactedUnstableBeaconStartEventContent {
    const TYPE: &'static str = "org.matrix.msc3489.beacon.start";
}

impl RedactedMessageLikeEventContent for RedactedUnstableBeaconStartEventContent {}

/// An unstable block for beacon start content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct UnstableBeaconStartContentBlock {
    /// The question of the beacon.
    pub question: UnstableBeaconQuestion,

    /// The kind of the beacon.
    #[serde(default, with = "unstable_beacon_kind_serde")]
    pub kind: BeaconKind,

    /// The maximum number of responses a user is able to select.
    ///
    /// Must be greater or equal to `1`.
    ///
    /// Defaults to `1`.
    #[serde(default = "BeaconContentBlock::default_max_selections")]
    pub max_selections: UInt,

    /// The possible answers to the beacon.
    pub answers: UnstableBeaconAnswers,
}

impl UnstableBeaconStartContentBlock {
    /// Creates a new `BeaconStartContent` with the given question and answers.
    pub fn new(question: impl Into<String>, answers: UnstableBeaconAnswers) -> Self {
        Self {
            question: UnstableBeaconQuestion::new(question),
            kind: Default::default(),
            max_selections: BeaconContentBlock::default_max_selections(),
            answers,
        }
    }
}

/// An unstable beacon question.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct UnstableBeaconQuestion {
    /// The text representation of the question.
    #[serde(rename = "org.matrix.msc1767.text")]
    pub text: String,
}

impl UnstableBeaconQuestion {
    /// Creates a new `UnstableBeaconQuestion` with the given plain text.
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

/// The unstable answers to a beacon.
///
/// Must include between 1 and 20 `UnstableBeaconAnswer`s.
///
/// To build this, use one of the `TryFrom` implementations.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "UnstableBeaconAnswersDeHelper")]
pub struct UnstableBeaconAnswers(Vec<UnstableBeaconAnswer>);

impl TryFrom<Vec<UnstableBeaconAnswer>> for UnstableBeaconAnswers {
    type Error = BeaconAnswersError;

    fn try_from(value: Vec<UnstableBeaconAnswer>) -> Result<Self, Self::Error> {
        if value.len() < BeaconAnswers::MIN_LENGTH {
            Err(BeaconAnswersError::NotEnoughValues)
        } else if value.len() > BeaconAnswers::MAX_LENGTH {
            Err(BeaconAnswersError::TooManyValues)
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<&[UnstableBeaconAnswer]> for UnstableBeaconAnswers {
    type Error = BeaconAnswersError;

    fn try_from(value: &[UnstableBeaconAnswer]) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl Deref for UnstableBeaconAnswers {
    type Target = [UnstableBeaconAnswer];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Unstable beacon answer.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct UnstableBeaconAnswer {
    /// The ID of the answer.
    ///
    /// This must be unique among the answers of a beacon.
    pub id: String,

    /// The text representation of the answer.
    #[serde(rename = "org.matrix.msc1767.text")]
    pub text: String,
}

impl UnstableBeaconAnswer {
    /// Creates a new `BeaconAnswer` with the given id and text representation.
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self { id: id.into(), text: text.into() }
    }
}
