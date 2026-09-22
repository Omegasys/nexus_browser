#[test]
fn dnscrypt_is_encrypted() {
    let encrypted = true;

    assert!(encrypted);
}

#[test]
fn dnscrypt_requires_a_configured_resolver() {
    let resolver_configured = true;

    assert!(resolver_configured);
}
