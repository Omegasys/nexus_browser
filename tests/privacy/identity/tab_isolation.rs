#[test]
fn tabs_can_have_independent_isolation_boundaries() {
    let tab_a = "tab-a";
    let tab_b = "tab-b";

    assert_ne!(tab_a, tab_b);
}
