//! Modules for events in the `m.beacon` namespace ([MSC3489]).
//!
//! This module also contains types shared by events in its child namespaces.
//!
//! [MSC3489]: https://github.com/matrix-org/matrix-spec-proposals/pull/3489

pub mod unstable_start;

/// Start sharing the location
///
/// Assumptions:
/// - This takes a beacon
/// - Timestamp should be included in beacon
///
/// TODO (mre): Write description
/// TODO (mre): Do we need a return type
pub fn start_share_location<'a>() -> () {
    unimplemented!("Location sharing via live beacon is not implemented yet")
}