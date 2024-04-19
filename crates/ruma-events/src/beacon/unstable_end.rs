//! Types for the `org.matrix.msc3489.beacon.end` event, the unstable version of `m.beacon.end`.

use ruma_common::OwnedEventId;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use crate::relation::Reference;

/// The payload for an unstable beacon end event.
///
/// This type can be generated from the unstable beacon start and beacon response events with
/// [`OriginalSyncUnstableBeaconStartEvent::compile_results()`].
///
/// This is the event content that should be sent for room versions that don't support extensible
/// events. As of Matrix 1.7, none of the stable room versions (1 through 10) support extensible
/// events.
///
/// To send a beacon end event for a room version that supports extensible events, use
/// [`BeaconEndEventContent`].
///
/// [`OriginalSyncUnstableBeaconStartEvent::compile_results()`]: super::unstable_start::OriginalSyncUnstableBeaconStartEvent::compile_results
/// [`BeaconEndEventContent`]: super::end::BeaconEndEventContent
#[derive(Clone, Debug, Serialize, Deserialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "org.matrix.msc3489.beacon.end", kind = MessageLike)]
pub struct UnstableBeaconEndEventContent {
    /// The text representation of the results.
    #[serde(rename = "org.matrix.msc1767.text")]
    pub text: String,

    /// The beacon end content.
    #[serde(default, rename = "org.matrix.msc3489.beacon.end")]
    pub beacon_end: UnstableBeaconEndContentBlock,

    /// Information about the beacon start event this responds to.
    #[serde(rename = "m.relates_to")]
    pub relates_to: Reference,
}

impl UnstableBeaconEndEventContent {
    /// Creates a new `BeaconEndEventContent` with the given fallback representation and
    /// that responds to the given beacon start event ID.
    pub fn new(text: impl Into<String>, beacon_start_id: OwnedEventId) -> Self {
        Self {
            text: text.into(),
            beacon_end: UnstableBeaconEndContentBlock {},
            relates_to: Reference::new(beacon_start_id),
        }
    }
}

/// A block for the results of a beacon.
///
/// This is currently an empty struct.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct UnstableBeaconEndContentBlock {}
