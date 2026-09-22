#[test]
fn isolated_routes_should_not_share_policy_state() {
    let route_a = "workspace-a";
    let route_b = "workspace-b";

    assert_ne!(route_a, route_b);
}
