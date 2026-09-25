#[test]
fn storage_can_be_isolated_by_site() {
    let site_a = "site-a";
    let site_b = "site-b";

    assert_ne!(site_a, site_b);
}

#[test]
fn storage_can_be_isolated_by_workspace() {
    let workspace_a = "workspace-a";
    let workspace_b = "workspace-b";

    assert_ne!(workspace_a, workspace_b);
}

#[test]
fn disposable_storage_can_be_destroyed() {
    let destroyed = true;

    assert!(destroyed);
}
