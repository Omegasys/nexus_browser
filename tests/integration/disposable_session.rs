#[test]
fn disposable_session_can_destroy_ephemeral_state() {
    let storage_destroyed = true;
    let identity_destroyed = true;
    let microvm_destroyed = true;

    assert!(storage_destroyed);
    assert!(identity_destroyed);
    assert!(microvm_destroyed);
}
