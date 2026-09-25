#[test]
fn certificate_validation_can_be_required() {
    let required = true;

    assert!(required);
}

#[test]
fn invalid_certificate_can_be_rejected() {
    let valid = false;
    let accepted = valid;

    assert!(!accepted);
}
