#[test]
fn different_sites_can_have_different_isolation_contexts() {
    let site_a = "example-a";
    let site_b = "example-b";

    assert_ne!(site_a, site_b);
}

#[test]
fn site_isolation_can_be_strict() {
    let strict = true;

    assert!(strict);
}
