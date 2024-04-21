#![cfg(feature = "unstable-msc3489")]


use std::time::Duration;

use js_int::uint;
use ruma_common::MilliSecondsSinceUnixEpoch;
use ruma_events::{
    beacon::unstable_start::BeaconInfoEventContent,
    location::{AssetContent, AssetType},
};

#[test]
fn unstable_beacon_start_event_content_serialization() {
    let event_content = BeaconInfoEventContent {
        description: Some("Alice's location".to_owned()),
        live: true,
        ts: Some(MilliSecondsSinceUnixEpoch(uint!(1_636_829_458))),
        timeout: Duration::from_secs(60),
        asset: AssetContent { type_: AssetType::Self_ },
    };

    assert_eq!(
        serde_json::to_value(&event_content).unwrap(),
        serde_json::json!({
            "org.matrix.msc3488.ts": 1_636_829_458,
            "org.matrix.msc3488.asset": {
                "type": "m.self"
            },
            "org.matrix.msc3488.timeout": 60_000,
            "description": "Alice's location",
            "live": true
        })
    );
}
