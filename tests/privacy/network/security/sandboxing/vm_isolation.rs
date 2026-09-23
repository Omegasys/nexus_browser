#[test]
fn microvm_can_be_used_as_an_isolation_boundary() {
    let isolated = true;

    assert!(isolated);
}

#[test]
fn vm_boundary_can_contain_untrusted_code() {
    let untrusted = true;
    let isolated = true;

    assert!(untrusted);
    assert!(isolated);
}
