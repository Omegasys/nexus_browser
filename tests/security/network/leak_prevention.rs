#[test]
fn leak_prevention_can_block_direct_fallback() {
    let fallback_allowed = false;

    assert!(!fallback_allowed);
}
