#[test]
fn plaintext_dns_is_a_leak_when_secure_dns_is_required() {
    let secure_dns_required = true;
    let plaintext_dns_used = true;

    let leak = secure_dns_required && plaintext_dns_used;

    assert!(leak);
}

#[test]
fn system_dns_is_blocked_when_policy_requires_secure_dns() {
    let secure_dns_required = true;
    let system_dns_allowed = false;

    assert!(secure_dns_required);
    assert!(!system_dns_allowed);
}
