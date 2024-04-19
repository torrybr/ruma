//! Types for the `org.matrix.msc3489.beacon.start` event, the unstable version of `m.beacon.start`.

use std::ops::Deref;

use js_int::UInt;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

mod content_serde;

use ruma_common::{MilliSecondsSinceUnixEpoch, OwnedEventId};

use crate::{
    beacon::UnstableBeaconStartEventContent, relation::Replacement,
    room::message::RelationWithoutReplacement, EventContent, MessageLikeEventContent,
    MessageLikeEventType, RedactContent, RedactedMessageLikeEventContent, StaticEventContent,
};

impl RedactContent for UnstableBeaconStartEventContent {
    type Redacted = RedactedUnstableBeaconStartEventContent;

    fn redact(self, _version: &crate::RoomVersionId) -> Self::Redacted {
        RedactedUnstableBeaconStartEventContent::default()
    }
}

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

impl RedactedMessageLikeEventContent for RedactedUnstableBeaconStartEventContent {}

impl EventContent for RedactedUnstableBeaconStartEventContent {
    type EventType = MessageLikeEventType;

    fn event_type(&self) -> Self::EventType {
        MessageLikeEventType::UnstablePollStart
    }
}

// TODO (mre): We probably don't need this as redacting a beacon event is not useful
// Remove this if we don't need it
// impl RedactContent for UnstableBeaconStartEventContent {
//     type Redacted = RedactedUnstableBeaconStartEventContent;

//     fn redact(self, _version: &crate::RoomVersionId) -> Self::Redacted {
//         RedactedUnstableBeaconStartEventContent::default()
//     }
// }

// impl From<NewUnstableBeaconStartEventContent> for UnstableBeaconStartEventContent {
//     fn from(value: NewUnstableBeaconStartEventContent) -> Self {
//         Self::New(value)
//     }
// }

impl OriginalSyncUnstableBeaconStartEvent {
    /// Compile the results for this beacon with the given response into an
    /// `UnstableBeaconEndEventContent`.
    ///
    /// It generates a default text representation of the results in English.
    ///
    /// This uses [`start_unstable_share_location()`] internally.
    pub fn start_share_location<'a>(&'a self) {
        // let beacon_start = self.content.beacon_start();

        unimplemented!("start_share_location")

        // let full_results =
        //     start_unstable_share_location(beacon_start, Some(MilliSecondsSinceUnixEpoch::now()));
        // let results =
        //     full_results.into_iter().map(|(id, users)| (id, users.len())).collect::<Vec<_>>();

        // // Get the text representation of the best answers.
        // let answers = beacon_start
        //     .answers
        //     .iter()
        //     .map(|a| (a.id.as_str(), a.text.as_str()))
        //     .collect::<Vec<_>>();
        // let plain_text = generate_beacon_end_fallback_text(&answers, results.into_iter());

        // UnstableBeaconEndEventContent::new(plain_text, self.event_id.clone())
    }
}

// /// A new unstable beacon start event.
// #[derive(Clone, Debug, Serialize)]
// #[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
// pub struct NewUnstableBeaconStartEventContent {
//     /// The beacon content of the message.
//     #[serde(rename = "org.matrix.msc3489.beacon.start")]
//     pub beacon_start: UnstableBeaconStartContentBlock,

//     /// Text representation of the message, for clients that don't support beacons.
//     #[serde(rename = "org.matrix.msc1767.text")]
//     pub text: Option<String>,

//     /// Information about related messages.
//     #[serde(rename = "m.relates_to", skip_serializing_if = "Option::is_none")]
//     pub relates_to: Option<RelationWithoutReplacement>,
// }

// impl NewUnstableBeaconStartEventContent {
//     /// Creates a `NewUnstableBeaconStartEventContent` with the given beacon content.
//     pub fn new(beacon_start: UnstableBeaconStartContentBlock) -> Self {
//         Self { beacon_start, text: None, relates_to: None }
//     }

//     /// Creates a `NewUnstableBeaconStartEventContent` with the given plain text fallback
//     /// representation and beacon content.
//     pub fn plain_text(
//         text: impl Into<String>,
//         beacon_start: UnstableBeaconStartContentBlock,
//     ) -> Self {
//         Self { beacon_start, text: Some(text.into()), relates_to: None }
//     }
// }

// impl EventContent for NewUnstableBeaconStartEventContent {
//     type EventType = MessageLikeEventType;

//     fn event_type(&self) -> Self::EventType {
//         MessageLikeEventType::UnstableBeaconStart
//     }
// }

// impl StaticEventContent for NewUnstableBeaconStartEventContent {
//     const TYPE: &'static str = "org.matrix.msc3489.beacon.start";
// }

// impl MessageLikeEventContent for NewUnstableBeaconStartEventContent {}

// /// Form of [`NewUnstableBeaconStartEventContent`] without relation.
// ///
// /// To construct this type, construct a [`NewUnstableBeaconStartEventContent`] and then use one
// of /// its `::from()` / `.into()` methods.
// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
// pub struct NewUnstableBeaconStartEventContentWithoutRelation {
//     /// The beacon content of the message.
//     #[serde(rename = "org.matrix.msc3489.beacon.start")]
//     pub beacon_start: UnstableBeaconStartContentBlock,

//     /// Text representation of the message, for clients that don't support beacons.
//     #[serde(rename = "org.matrix.msc1767.text")]
//     pub text: Option<String>,
// }

// impl From<NewUnstableBeaconStartEventContent>
//     for NewUnstableBeaconStartEventContentWithoutRelation
// {
//     fn from(value: NewUnstableBeaconStartEventContent) -> Self {
//         let NewUnstableBeaconStartEventContent { beacon_start, text, .. } = value;
//         Self { beacon_start, text }
//     }
// }

// /// Redacted form of UnstableBeaconStartEventContent
// #[derive(Clone, Debug, Default, Serialize, Deserialize)]
// #[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
// pub struct RedactedUnstableBeaconStartEventContent {}

// impl RedactedUnstableBeaconStartEventContent {
//     /// Creates an empty RedactedUnstableBeaconStartEventContent.
//     pub fn new() -> RedactedUnstableBeaconStartEventContent {
//         Self::default()
//     }
// }

// impl EventContent for RedactedUnstableBeaconStartEventContent {
//     type EventType = MessageLikeEventType;

//     fn event_type(&self) -> Self::EventType {
//         MessageLikeEventType::UnstableBeaconStart
//     }
// }

// impl StaticEventContent for RedactedUnstableBeaconStartEventContent {
//     const TYPE: &'static str = "org.matrix.msc3489.beacon.start";
// }

// impl RedactedMessageLikeEventContent for RedactedUnstableBeaconStartEventContent {}

// /// An unstable block for beacon start content.
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
// pub struct UnstableBeaconStartContentBlock {
//     /// The question of the beacon.
//     pub question: UnstableBeaconQuestion,

//     /// The maximum number of responses a user is able to select.
//     ///
//     /// Must be greater or equal to `1`.
//     ///
//     /// Defaults to `1`.
//     #[serde(default = "BeaconContentBlock::default_max_selections")]
//     pub max_selections: UInt,

//     /// The possible answers to the beacon.
//     pub answers: UnstableBeaconAnswers,
// }

// impl UnstableBeaconStartContentBlock {
//     /// Creates a new `BeaconStartContent` with the given question and answers.
//     pub fn new(question: impl Into<String>, answers: UnstableBeaconAnswers) -> Self {
//         Self {
//             question: UnstableBeaconQuestion::new(question),
//             kind: Default::default(),
//             max_selections: BeaconContentBlock::default_max_selections(),
//             answers,
//         }
//     }
// }
