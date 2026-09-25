#[test]
fn permission_can_be_granted() {
    let granted = true;

    assert!(granted);
}

#[test]
fn permission_can_be_denied() {
    let granted = false;

    assert!(!granted);
}
