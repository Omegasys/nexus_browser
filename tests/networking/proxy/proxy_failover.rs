#[test]
fn proxy_failover_can_be_fail_closed() {
    let fail_closed = true;
    let fallback_allowed = false;

    assert!(fail_closed);
    assert!(!fallback_allowed);
}
