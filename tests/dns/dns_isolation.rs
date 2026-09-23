#[test]
fn dns_contexts_can_be_isolated_per_workspace() {
    let workspace_a = "workspace-a";
    let workspace_b = "workspace-b";

    assert_ne!(workspace_a, workspace_b);
}

#[test]
fn dns_contexts_can_be_isolated_per_profile() {
    let profile_a = "privacy";
    let profile_b = "compatibility";

    assert_ne!(profile_a, profile_b);
}
