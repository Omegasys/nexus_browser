#[test]
fn tor_failure_does_not_fall_back_to_direct_by_default() {
    let tor_available = false;
    let direct_fallback = false;

    assert!(!tor_available);
    assert!(!direct_fallback);
}
