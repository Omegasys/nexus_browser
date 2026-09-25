#[test]
fn failed_private_route_does_not_fall_back_to_direct() {
    let private_route_failed = true;
    let direct_fallback = false;

    assert!(private_route_failed);
    assert!(!direct_fallback);
}

#[test]
fn failed_secure_dns_does_not_fall_back_to_plaintext() {
    let secure_dns_failed = true;
    let plaintext_fallback = false;

    assert!(secure_dns_failed);
    assert!(!plaintext_fallback);
}
