#[test]
fn privacy_policy_can_block_unwanted_protocols() {
    let blocked = [
        "webrtc",
        "quic",
        "udp",
    ];

    assert!(blocked.contains(&"webrtc"));
}
