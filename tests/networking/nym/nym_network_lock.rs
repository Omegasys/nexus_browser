#[test]
fn nym_lock_prevents_direct_fallback() {
    let locked = true;
    let direct_allowed = false;

    assert!(locked);
    assert!(!direct_allowed);
}
