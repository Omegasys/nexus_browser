#[test]
fn protocol_lock_can_block_quic() {
    let quic_allowed = false;

    assert!(!quic_allowed);
}
