//! Modules for events in the `m.beacon` namespace ([MSC3489]).
//!
//! This module also contains types shared by events in its child namespaces.
//!
//! [MSC3489]: https://github.com/matrix-org/matrix-spec-proposals/pull/3489

use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Deref,
};

use indexmap::IndexMap;
use js_int::{uint, UInt};
use ruma_common::{MilliSecondsSinceUnixEpoch, UserId};

// use self::unstable_start::UnstableBeaconStartContentBlock;

pub mod unstable_start;

/// Start sharing the location
///
/// Assumptions:
/// - This takes a beacon
/// - Timestamp should be included in beacon
///
/// TODO (mre): Write description
// pub fn start_share_location<'a>(beacon: &'a UnstableBeaconStartEventContent) -> () {
pub fn start_share_location<'a>() -> () {
    unimplemented!("Location sharing via live beacon is not implemented yet")

    // aggregate_results(poll.answers.iter().map(|a| a.id.as_str()), users_selections)
}
