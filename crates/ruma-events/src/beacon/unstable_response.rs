//! Types for the `org.matrix.msc3489.beacon.response` event, the unstable version of
//! `m.beacon.response`.

use ruma_common::OwnedEventId;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use super::{
    unstable_start::UnstableBeaconStartContentBlock, validate_selections, BeaconResponseData,
};
use crate::relation::Reference;

/// The payload for an unstable beacon response event.
///
/// This is the event content that should be sent for room versions that don't support extensible
/// events. As of Matrix 1.7, none of the stable room versions (1 through 10) support extensible
/// events.
///
/// To send a beacon response event for a room version that supports extensible events, use
/// [`BeaconResponseEventContent`].
///
/// [`BeaconResponseEventContent`]: super::response::BeaconResponseEventContent
#[derive(Clone, Debug, Serialize, Deserialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "org.matrix.msc3489.beacon.response", kind = MessageLike)]
pub struct UnstableBeaconResponseEventContent {
    /// The response's content.
    #[serde(rename = "org.matrix.msc3489.beacon.response")]
    pub beacon_response: UnstableBeaconResponseContentBlock,

    /// Information about the beacon start event this responds to.
    #[serde(rename = "m.relates_to")]
    pub relates_to: Reference,
}

impl UnstableBeaconResponseEventContent {
    /// Creates a new `UnstableBeaconResponseEventContent` that responds to the given beacon start
    /// event ID, with the given answers.
    pub fn new(answers: Vec<String>, beacon_start_id: OwnedEventId) -> Self {
        Self {
            beacon_response: UnstableBeaconResponseContentBlock::new(answers),
            relates_to: Reference::new(beacon_start_id),
        }
    }
}

impl OriginalSyncUnstableBeaconResponseEvent {
    /// Get the data from this response necessary to compile beacon results.
    pub fn data(&self) -> BeaconResponseData<'_> {
        BeaconResponseData {
            sender: &self.sender,
            origin_server_ts: self.origin_server_ts,
            selections: &self.content.beacon_response.answers,
        }
    }
}

impl OriginalUnstableBeaconResponseEvent {
    /// Get the data from this response necessary to compile beacon results.
    pub fn data(&self) -> BeaconResponseData<'_> {
        BeaconResponseData {
            sender: &self.sender,
            origin_server_ts: self.origin_server_ts,
            selections: &self.content.beacon_response.answers,
        }
    }
}

/// An unstable block for beacon response content.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct UnstableBeaconResponseContentBlock {
    /// The selected answers for the response.
    pub answers: Vec<String>,
}

impl UnstableBeaconResponseContentBlock {
    /// Creates a new `UnstableBeaconResponseContentBlock` with the given answers.
    pub fn new(answers: Vec<String>) -> Self {
        Self { answers }
    }

    /// Validate these selections against the given `UnstableBeaconStartContentBlock`.
    ///
    /// Returns the list of valid selections in this `UnstableBeaconResponseContentBlock`, or `None`
    /// if there is no valid selection.
    pub fn validate<'a>(
        &'a self,
        beacon: &UnstableBeaconStartContentBlock,
    ) -> Option<impl Iterator<Item = &'a str>> {
        let answer_ids = beacon.answers.iter().map(|a| a.id.as_str()).collect();
        validate_selections(&answer_ids, beacon.max_selections, &self.answers)
    }
}

impl From<Vec<String>> for UnstableBeaconResponseContentBlock {
    fn from(value: Vec<String>) -> Self {
        Self::new(value)
    }
}
