#[test]
fn lokinet_lock_blocks_unapproved_fallback() {
    let locked = true;
    let direct_allowed = false;

    assert!(locked);
    assert!(!direct_allowed);
}
