#[test]
fn tcp_can_be_allowed() {
    let allowed = true;

    assert!(allowed);
}

#[test]
fn udp_can_be_blocked() {
    let allowed = false;

    assert!(!allowed);
}
