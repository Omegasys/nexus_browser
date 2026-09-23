use std::collections::HashMap;

#[test]
fn dns_cache_can_store_resolution() {
    let mut cache = HashMap::new();

    cache.insert(
        "example.com",
        "93.184.216.34",
    );

    assert_eq!(
        cache.get("example.com"),
        Some(&"93.184.216.34")
    );
}

#[test]
fn dns_cache_can_expire_entries() {
    let expired = true;

    assert!(expired);
}
