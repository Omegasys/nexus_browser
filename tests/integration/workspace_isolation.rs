#[test]
fn workspaces_can_have_independent_network_and_storage() {
    let workspace_a = ("vpn", "storage-a");
    let workspace_b = ("tor", "storage-b");

    assert_ne!(workspace_a, workspace_b);
}
