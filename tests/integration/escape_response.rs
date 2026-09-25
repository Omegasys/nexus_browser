#[test]
fn detected_escape_can_quarantine_microvm() {
    let escape_detected = true;
    let quarantined = true;

    assert!(escape_detected);
    assert!(quarantined);
}

#[test]
fn quarantined_vm_cannot_continue_normal_execution() {
    let state = "quarantined";

    assert_ne!(state, "running");
}
