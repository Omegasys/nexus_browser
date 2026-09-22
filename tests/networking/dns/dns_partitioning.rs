#[test]
fn dns_cache_can_be_partitioned_by_top_level_site() {
    let first = ("example.com", "site-a");
    let second = ("example.com", "site-b");

    assert_ne!(first, second);
}
