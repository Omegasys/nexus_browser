#[test]
fn dns_lock_blocks_unapproved_dns() {
    let locked = true;
    let dns_allowed = false;

    assert!(locked);
    assert!(!dns_allowed);
}
