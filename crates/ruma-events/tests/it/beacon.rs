#![cfg(feature = "unstable-msc3489")]

use ruma_events::beacon::UnstableBeaconStartEventContent;

#[test]
fn beacon_starts_not_live() {
    let timeout = std::time::Duration::from_secs(60);

    let beacon = UnstableBeaconStartEventContent::new(None, timeout);

    assert_eq!(beacon.live, false);
}
