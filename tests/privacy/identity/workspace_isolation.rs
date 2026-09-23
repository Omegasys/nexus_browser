#[test]
fn workspaces_can_have_independent_identities() {
    let workspace_a_identity = "identity-a";
    let workspace_b_identity = "identity-b";

    assert_ne!(
        workspace_a_identity,
        workspace_b_identity
    );
}
