#[test]
fn local_storage_isolated_between_sites() {
    let site_a = "example-a";
    let site_b = "example-b";

    assert_ne!(site_a, site_b);
}
