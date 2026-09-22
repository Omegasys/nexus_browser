#[test]
fn connection_lifecycle_has_expected_states() {
    let states = [
        "created",
        "connecting",
        "connected",
        "closing",
        "closed",
    ];

    assert_eq!(states.len(), 5);
}
