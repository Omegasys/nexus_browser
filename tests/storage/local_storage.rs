#[test]
fn local_storage_can_store_values() {
    let mut storage = std::collections::HashMap::new();

    storage.insert(
        "theme",
        "dark",
    );

    assert_eq!(
        storage.get("theme"),
        Some(&"dark")
    );
}

#[test]
fn local_storage_can_remove_values() {
    let mut storage = std::collections::HashMap::new();

    storage.insert("theme", "dark");
    storage.remove("theme");

    assert!(!storage.contains_key("theme"));
}
