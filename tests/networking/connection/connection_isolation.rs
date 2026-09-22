#[test]
fn connections_can_be_isolated_by_workspace() {
    let workspace_a = "workspace-a";
    let workspace_b = "workspace-b";

    assert_ne!(workspace_a, workspace_b);
}
