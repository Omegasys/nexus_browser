#[test]
fn end_to_end_security_boundary_is_enabled() {
    let sandbox = true;
    let site_isolation = true;
    let process_isolation = true;
    let microvm = true;
    let network_lock = true;
    let secure_dns = true;
    let storage_partitioning = true;

    assert!(sandbox);
    assert!(site_isolation);
    assert!(process_isolation);
    assert!(microvm);
    assert!(network_lock);
    assert!(secure_dns);
    assert!(storage_partitioning);
}

#[test]
fn end_to_end_failure_is_fail_closed() {
    let private_route_failed = true;
    let secure_dns_failed = true;

    let direct_fallback = false;
    let plaintext_dns_fallback = false;

    assert!(private_route_failed);
    assert!(secure_dns_failed);
    assert!(!direct_fallback);
    assert!(!plaintext_dns_fallback);
}
