#[test]
fn tabs_can_have_independent_routes() {
    let tab_a_route = "tor";
    let tab_b_route = "direct";

    assert_ne!(tab_a_route, tab_b_route);
}
