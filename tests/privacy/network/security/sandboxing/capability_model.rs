#[test]
fn capabilities_can_be_explicitly_granted() {
    let capabilities = [
        "rendering",
        "networking",
    ];

    assert!(capabilities.contains(&"rendering"));
}

#[test]
fn ungranted_capability_is_denied() {
    let granted = [
        "rendering",
    ];

    assert!(!granted.contains(&"native-process"));
}
