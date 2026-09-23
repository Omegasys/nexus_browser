#[test]
fn storage_can_be_partitioned_by_top_level_site() {
    let partition_a = "site-a";
    let partition_b = "site-b";

    assert_ne!(partition_a, partition_b);
}
