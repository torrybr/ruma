//! `Serialize` and `Deserialize` helpers for unstable beacon kind (MSC3381).

use std::borrow::Cow;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{beacon::start::BeaconKind, PrivOwnedStr};

/// Serializes a BeaconKind using the unstable prefixes.
pub(super) fn serialize<S>(kind: &BeaconKind, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = match kind {
        BeaconKind::Undisclosed => "org.matrix.msc3489.beacon.undisclosed",
        BeaconKind::Disclosed => "org.matrix.msc3489.beacon.disclosed",
        BeaconKind::_Custom(s) => &s.0,
    };

    s.serialize(serializer)
}

/// Deserializes a BeaconKind using the unstable prefixes.
pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<BeaconKind, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Cow::<'_, str>::deserialize(deserializer)?;

    let kind = match &*s {
        "org.matrix.msc3489.beacon.undisclosed" => BeaconKind::Undisclosed,
        "org.matrix.msc3489.beacon.disclosed" => BeaconKind::Disclosed,
        _ => BeaconKind::_Custom(PrivOwnedStr(s.into())),
    };

    Ok(kind)
}
