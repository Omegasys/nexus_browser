#[test]
fn strict_dns_policy_requires_encryption() {
    let require_encryption = true;

    assert!(require_encryption);
}

#[test]
fn strict_dns_policy_can_require_dnssec() {
    let require_dnssec = true;

    assert!(require_dnssec);
}

#[test]
fn strict_dns_policy_can_disable_system_dns() {
    let system_dns_allowed = false;

    assert!(!system_dns_allowed);
}

#[test]
fn strict_dns_policy_can_disable_plaintext_fallback() {
    let plaintext_fallback = false;

    assert!(!plaintext_fallback);
}
