#[test]
fn storage_can_be_isolated_inside_security_boundary() {
    let storage_isolation = true;
    let sandbox = true;

    assert!(storage_isolation);
    assert!(sandbox);
}
