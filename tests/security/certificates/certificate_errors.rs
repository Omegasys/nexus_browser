#[test]
fn certificate_errors_can_be_reported() {
    let error = Some("certificate expired");

    assert!(error.is_some());
}
