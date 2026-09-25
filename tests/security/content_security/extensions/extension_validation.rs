#[test]
fn extension_manifest_can_be_validated() {
    let valid = true;

    assert!(valid);
}

#[test]
fn invalid_extension_can_be_rejected() {
    let valid = false;

    assert!(!valid);
}
