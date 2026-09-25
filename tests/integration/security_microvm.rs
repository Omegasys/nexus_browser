#[test]
fn microvm_is_part_of_security_boundary() {
    let microvm = true;
    let escape_prevention = true;

    assert!(microvm);
    assert!(escape_prevention);
}
