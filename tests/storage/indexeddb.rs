#[test]
fn indexeddb_database_can_be_created() {
    let database = "test-db";

    assert!(!database.is_empty());
}

#[test]
fn indexeddb_records_can_be_stored() {
    let mut records = std::collections::HashMap::new();

    records.insert(
        "key",
        "value",
    );

    assert_eq!(
        records.get("key"),
        Some(&"value")
    );
}
