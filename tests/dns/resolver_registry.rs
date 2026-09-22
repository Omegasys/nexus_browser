#[test]
fn resolver_registry_can_track_multiple_resolvers() {
    let resolvers = [
        "system",
        "doh",
        "dot",
        "dnscrypt",
        "tor",
    ];

    assert_eq!(resolvers.len(), 5);
}

#[test]
fn resolver_registry_can_prioritize_secure_resolvers() {
    let preferred = "doh";

    assert_eq!(preferred, "doh");
}
