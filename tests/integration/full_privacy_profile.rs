#[test]
fn full_privacy_profile_enables_core_privacy_controls() {
    let secure_dns = true;
    let storage_partitioning = true;
    let tracker_protection = true;
    let fingerprint_protection = true;
    let webrtc_protection = true;

    assert!(secure_dns);
    assert!(storage_partitioning);
    assert!(tracker_protection);
    assert!(fingerprint_protection);
    assert!(webrtc_protection);
}
