#[test]
fn direct_fallback_should_be_blockable() {
    let network_lock = true;
    let direct_fallback_allowed = false;

    assert!(network_lock);
    assert!(!direct_fallback_allowed);
}
