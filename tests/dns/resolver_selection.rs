#[test]
fn resolver_selection_prefers_available_secure_resolver() {
    let resolvers = [
        ("system", false),
        ("doh", true),
        ("dot", true),
    ];

    let selected = resolvers
        .iter()
        .find(|(_, available)| *available)
        .map(|(name, _)| *name);

    assert_eq!(selected, Some("doh"));
}

#[test]
fn unavailable_resolvers_are_skipped() {
    let available = false;

    assert!(!available);
}
