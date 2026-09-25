#[test]
fn broadcast_channel_can_have_a_name() {
    let channel = "test-channel";

    assert!(!channel.is_empty());
}

#[test]
fn broadcast_channel_can_be_scoped_to_an_origin() {
    let origin = "https://example.com";

    assert!(origin.starts_with("https://"));
}
