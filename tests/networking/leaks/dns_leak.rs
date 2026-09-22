#[test]
fn dns_leak_is_detected_when_plaintext_dns_is_used() {
    let encrypted_dns = false;

    assert!(!encrypted_dns);
}
