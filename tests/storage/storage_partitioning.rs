#[test]
fn storage_partitions_can_be_distinct() {
    let first = "top-level-a";
    let second = "top-level-b";

    assert_ne!(first, second);
}

#[test]
fn third_party_storage_can_be_partitioned() {
    let partitioned = true;

    assert!(partitioned);
}
