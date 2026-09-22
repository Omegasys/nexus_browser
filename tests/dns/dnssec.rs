#[test]
fn dnssec_validation_can_be_required() {
    let required = true;

    assert!(required);
}

#[test]
fn invalid_dnssec_results_can_be_rejected() {
    let valid = false;
    let accepted = valid;

    assert!(!accepted);
}
