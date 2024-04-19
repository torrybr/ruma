//! Types for the `m.beacon.end` event.

use std::{
    collections::{btree_map, BTreeMap},
    ops::Deref,
};

use js_int::UInt;
use ruma_common::OwnedEventId;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use crate::{message::TextContentBlock, relation::Reference};

/// The payload for a beacon end event.
///
/// This type can be generated from the beacon start and beacon response events with
/// [`OriginalSyncBeaconStartEvent::compile_results()`].
///
/// This is the event content that should be sent for room versions that support extensible events.
/// As of Matrix 1.7, none of the stable room versions (1 through 10) support extensible events.
///
/// To send a beacon end event for a room version that does not support extensible events, use
/// [`UnstableBeaconEndEventContent`].
///
/// [`OriginalSyncBeaconStartEvent::compile_results()`]: super::start::OriginalSyncBeaconStartEvent::compile_results
/// [`UnstableBeaconEndEventContent`]: super::unstable_end::UnstableBeaconEndEventContent
#[derive(Clone, Debug, Serialize, Deserialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.beacon.end", kind = MessageLike)]
pub struct BeaconEndEventContent {
    /// The text representation of the results.
    #[serde(rename = "m.text")]
    pub text: TextContentBlock,

    /// The sender's perspective of the results.
    #[serde(rename = "m.beacon.results", skip_serializing_if = "Option::is_none")]
    pub beacon_results: Option<BeaconResultsContentBlock>,

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

impl BeaconEndEventContent {
    /// Creates a new `BeaconEndEventContent` with the given fallback representation and
    /// that responds to the given beacon start event ID.
    pub fn new(text: TextContentBlock, beacon_start_id: OwnedEventId) -> Self {
        Self {
            text,
            beacon_results: None,
            #[cfg(feature = "unstable-msc3955")]
            automated: false,
            relates_to: Reference::new(beacon_start_id),
        }
    }

    /// Creates a new `BeaconEndEventContent` with the given plain text fallback representation and
    /// that responds to the given beacon start event ID.
    pub fn with_plain_text(plain_text: impl Into<String>, beacon_start_id: OwnedEventId) -> Self {
        Self {
            text: TextContentBlock::plain(plain_text),
            beacon_results: None,
            #[cfg(feature = "unstable-msc3955")]
            automated: false,
            relates_to: Reference::new(beacon_start_id),
        }
    }
}

/// A block for the results of a beacon.
///
/// This is a map of answer ID to number of votes.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct BeaconResultsContentBlock(BTreeMap<String, UInt>);

impl BeaconResultsContentBlock {
    /// Get these results sorted from the highest number of votes to the lowest.
    ///
    /// Returns a list of `(answer ID, number of votes)`.
    pub fn sorted(&self) -> Vec<(&str, UInt)> {
        let mut sorted = self.0.iter().map(|(id, count)| (id.as_str(), *count)).collect::<Vec<_>>();
        sorted.sort_by(|(_, a), (_, b)| b.cmp(a));
        sorted
    }
}

impl From<BTreeMap<String, UInt>> for BeaconResultsContentBlock {
    fn from(value: BTreeMap<String, UInt>) -> Self {
        Self(value)
    }
}

impl IntoIterator for BeaconResultsContentBlock {
    type Item = (String, UInt);
    type IntoIter = btree_map::IntoIter<String, UInt>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<(String, UInt)> for BeaconResultsContentBlock {
    fn from_iter<T: IntoIterator<Item = (String, UInt)>>(iter: T) -> Self {
        Self(BTreeMap::from_iter(iter))
    }
}

impl Deref for BeaconResultsContentBlock {
    type Target = BTreeMap<String, UInt>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
