#[test]
fn dns_cache_can_store_entries() {
    let mut cache = std::collections::HashMap::new();

    cache.insert(
        "example.com",
        "93.184.216.34",
    );

    assert_eq!(
        cache.get("example.com"),
        Some(&"93.184.216.34")
    );
}
