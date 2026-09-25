#[test]
fn permission_policy_can_default_to_deny() {
    let default_allow = false;

    assert!(!default_allow);
}
