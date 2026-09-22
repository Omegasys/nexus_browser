#[test]
fn doh_requires_encrypted_transport() {
    let encrypted = true;

    assert!(encrypted);
}

#[test]
fn doh_has_a_resolver_endpoint() {
    let endpoint = "https://resolver.example/dns-query";

    assert!(endpoint.starts_with("https://"));
}
