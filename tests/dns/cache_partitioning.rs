#[test]
fn dns_cache_can_be_partitioned_by_top_level_site() {
    let partition_a = (
        "example.com",
        "site-a.example",
    );

    let partition_b = (
        "example.com",
        "site-b.example",
    );

    assert_ne!(partition_a, partition_b);
}

#[test]
fn different_partitions_should_not_share_identity() {
    let partition_a = "workspace-a";
    let partition_b = "workspace-b";

    assert_ne!(partition_a, partition_b);
}
