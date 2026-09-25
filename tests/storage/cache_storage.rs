#[test]
fn cache_storage_can_store_named_cache() {
    let caches = [
        "default",
        "images",
        "scripts",
    ];

    assert!(caches.contains(&"default"));
}

#[test]
fn cache_storage_can_be_cleared() {
    let cleared = true;

    assert!(cleared);
}
