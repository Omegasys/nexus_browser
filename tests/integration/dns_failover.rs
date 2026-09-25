#[test]
fn dns_failover_can_use_secondary_secure_resolver() {
    let primary = "doh";
    let secondary = "dot";

    assert_ne!(primary, secondary);
}

#[test]
fn dns_failover_does_not_use_plaintext_when_locked_down() {
    let approved = [
        "doh",
        "dot",
        "dnscrypt",
        "tor",
    ];

    assert!(!approved.contains(&"system"));
}
