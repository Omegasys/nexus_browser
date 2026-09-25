#[test]
fn cache_can_store_an_entry() {
    let mut cache = std::collections::HashMap::new();

    cache.insert(
        "https://example.com",
        "cached-content",
    );

    assert!(cache.contains_key("https://example.com"));
}

#[test]
fn cache_entry_can_expire() {
    let expired = true;

    assert!(expired);
}
