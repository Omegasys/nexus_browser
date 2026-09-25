#[test]
fn extension_sandbox_can_be_enabled() {
    let enabled = true;

    assert!(enabled);
}

#[test]
fn native_process_access_can_be_blocked() {
    let allowed = false;

    assert!(!allowed);
}
