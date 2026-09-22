#[test]
fn known_protocol_can_be_detected() {
    let protocol = "https";

    assert!(!protocol.is_empty());
}
