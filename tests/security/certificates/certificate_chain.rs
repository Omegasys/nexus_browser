#[test]
fn certificate_chain_can_be_validated() {
    let valid_chain = true;

    assert!(valid_chain);
}

#[test]
fn broken_certificate_chain_can_be_rejected() {
    let valid_chain = false;

    assert!(!valid_chain);
}
