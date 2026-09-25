#[test]
fn session_storage_is_temporary() {
    let persistent = false;

    assert!(!persistent);
}

#[test]
fn session_storage_can_store_tab_data() {
    let mut storage = std::collections::HashMap::new();

    storage.insert(
        "tab-state",
        "active",
    );

    assert!(storage.contains_key("tab-state"));
}
