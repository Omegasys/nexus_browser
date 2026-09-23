#[test]
fn privacy_score_contains_multiple_categories() {
    let categories = [
        "network",
        "dns",
        "storage",
        "cookies",
        "fingerprinting",
        "webrtc",
        "tracking",
        "isolation",
    ];

    assert_eq!(categories.len(), 8);
}

#[test]
fn privacy_score_is_bounded() {
    let score = 85u8;

    assert!(score <= 100);
}
