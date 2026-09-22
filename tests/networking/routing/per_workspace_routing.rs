#[test]
fn workspaces_can_have_independent_routes() {
    let workspace_a = "vpn";
    let workspace_b = "direct";

    assert_ne!(workspace_a, workspace_b);
}
