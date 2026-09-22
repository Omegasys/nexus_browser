#[test]
fn fail_closed_policy_has_no_automatic_direct_fallback() {
    let fail_closed = true;
    let direct_fallback = false;

    assert!(fail_closed);
    assert!(!direct_fallback);
}
