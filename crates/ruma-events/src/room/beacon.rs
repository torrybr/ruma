//! Types for the `org.matrix.msc3489.beacon.start` event, the unstable version of `m.beacon.start`.

use std::time::Duration;

use js_int::UInt;
use ruma_common::{MilliSecondsSinceUnixEpoch, OwnedUserId};
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use crate::location::AssetContent;

/// `BeaconInfoStateEventContent` is a struct that represents the content of a beacon_info state
/// event. It contains information about a live location sharing event.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "org.matrix.msc3672.beacon_info", alias = "m.beacon_info", kind = State, state_key_type = OwnedUserId)]
pub struct BeaconInfoEventContent {
    /// The description of the location.
    ///
    /// It should be used to label the location on a map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// `live` is a boolean that should be true when a user starts sharing location.
    pub live: bool,

    /// `ts` is an optional `MilliSecondsSinceUnixEpoch` that represents the timestamp of the
    /// event.
    #[serde(rename = "org.matrix.msc3488.ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<MilliSecondsSinceUnixEpoch>,

    /// `timeout` represents the length of time in milliseconds that the location
    /// will be live. So the location will stop being shared at `m.ts + timeout` milliseconds
    /// since the epoch.
    #[serde(default, with = "ruma_common::serde::duration::ms")]
    pub timeout: Duration,

    /// `asset` is an `AssetContent` that this message refers to.
    #[serde(
        default,
        rename = "org.matrix.msc3488.asset",
        skip_serializing_if = "ruma_common::serde::is_default"
    )]
    pub asset: AssetContent,
}

impl BeaconInfoEventContent {
    /// Creates a new `BeaconInfoEventContent` with the given description, live, timeout and asset.
    pub fn new(description: Option<String>, timeout: Duration) -> Self {
        Self { description, live: false, ts: None, timeout, asset: Default::default() }
    }

    /// starts the beacon being live.
    pub fn start(&mut self) {
        self.live = true;
    }

    /// Stops the beacon from being live.
    pub fn stop(&mut self) {
        self.live = false;
    }

    /// Checks if the beacon is currently live.
    ///
    /// This method calculates the current time and compares it with the beacon's start time plus
    /// its timeout. If the beacon is not live or the current time is greater than the beacon's
    /// start time plus its timeout, it returns false, indicating that the beacon is not live.
    /// Otherwise, it returns true.
    pub fn is_live(&self) -> bool {
        self.live
            && self.ts.unwrap().get() + UInt::try_from(self.timeout.as_millis()).unwrap()
                < MilliSecondsSinceUnixEpoch::now().get()
    }
}

// impl RedactContent for UnstableBeaconInfoContent {
//     type Redacted = RedactedUnstableBeaconInfoContent;

//     fn redact(self, _version: &crate::RoomVersionId) -> Self::Redacted {
//         RedactedUnstableBeaconInfoContent::default()
//     }
// }

// /// Redacted form of UnstableBeaconInfoContent
// #[derive(Clone, Debug, Default, Serialize, Deserialize)]
// #[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
// pub struct RedactedUnstableBeaconInfoContent {}

// impl RedactedUnstableBeaconInfoContent {
//     /// Creates an empty RedactedUnstableBeaconInfoContent.
//     pub fn new() -> RedactedUnstableBeaconInfoContent {
//         Self::default()
//     }
// }

// impl RedactedMessageLikeEventContent for RedactedUnstableBeaconInfoContent {}

// impl EventContent for RedactedUnstableBeaconInfoContent {
//     type EventType = MessageLikeEventType;

//     fn event_type(&self) -> Self::EventType {
//         MessageLikeEventType::UnstablePollStart
//     }
// }

// TODO (mre): We probably don't need this as redacting a beacon event is not useful
// Remove this if we don't need it
// impl RedactContent for UnstableBeaconInfoContent {
//     type Redacted = RedactedUnstableBeaconInfoContent;

//     fn redact(self, _version: &crate::RoomVersionId) -> Self::Redacted {
//         RedactedUnstableBeaconInfoContent::default()
//     }
// }

// impl From<NewUnstableBeaconInfoContent> for UnstableBeaconInfoContent {
//     fn from(value: NewUnstableBeaconInfoContent) -> Self {
//         Self::New(value)
//     }
// }

impl OriginalSyncBeaconInfoEvent {
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
// pub struct NewUnstableBeaconInfoContent {
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

// impl NewUnstableBeaconInfoContent {
//     /// Creates a `NewUnstableBeaconInfoContent` with the given beacon content.
//     pub fn new(beacon_start: UnstableBeaconStartContentBlock) -> Self {
//         Self { beacon_start, text: None, relates_to: None }
//     }

//     /// Creates a `NewUnstableBeaconInfoContent` with the given plain text fallback
//     /// representation and beacon content.
//     pub fn plain_text(
//         text: impl Into<String>,
//         beacon_start: UnstableBeaconStartContentBlock,
//     ) -> Self {
//         Self { beacon_start, text: Some(text.into()), relates_to: None }
//     }
// }

// impl EventContent for NewUnstableBeaconInfoContent {
//     type EventType = MessageLikeEventType;

//     fn event_type(&self) -> Self::EventType {
//         MessageLikeEventType::UnstableBeaconStart
//     }
// }

// impl StaticEventContent for NewUnstableBeaconInfoContent {
//     const TYPE: &'static str = "org.matrix.msc3489.beacon.start";
// }

// impl MessageLikeEventContent for NewUnstableBeaconInfoContent {}

// /// Form of [`NewUnstableBeaconInfoContent`] without relation.
// ///
// /// To construct this type, construct a [`NewUnstableBeaconInfoContent`] and then use one
// of /// its `::from()` / `.into()` methods.
// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
// pub struct NewUnstableBeaconInfoContentWithoutRelation {
//     /// The beacon content of the message.
//     #[serde(rename = "org.matrix.msc3489.beacon.start")]
//     pub beacon_start: UnstableBeaconStartContentBlock,

//     /// Text representation of the message, for clients that don't support beacons.
//     #[serde(rename = "org.matrix.msc1767.text")]
//     pub text: Option<String>,
// }

// impl From<NewUnstableBeaconInfoContent>
//     for NewUnstableBeaconInfoContentWithoutRelation
// {
//     fn from(value: NewUnstableBeaconInfoContent) -> Self {
//         let NewUnstableBeaconInfoContent { beacon_start, text, .. } = value;
//         Self { beacon_start, text }
//     }
// }

// /// Redacted form of UnstableBeaconInfoContent
// #[derive(Clone, Debug, Default, Serialize, Deserialize)]
// #[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
// pub struct RedactedUnstableBeaconInfoContent {}

// impl RedactedUnstableBeaconInfoContent {
//     /// Creates an empty RedactedUnstableBeaconInfoContent.
//     pub fn new() -> RedactedUnstableBeaconInfoContent {
//         Self::default()
//     }
// }

// impl EventContent for RedactedUnstableBeaconInfoContent {
//     type EventType = MessageLikeEventType;

//     fn event_type(&self) -> Self::EventType {
//         MessageLikeEventType::UnstableBeaconStart
//     }
// }

// impl StaticEventContent for RedactedUnstableBeaconInfoContent {
//     const TYPE: &'static str = "org.matrix.msc3489.beacon.start";
// }

// impl RedactedMessageLikeEventContent for RedactedUnstableBeaconInfoContent {}

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
