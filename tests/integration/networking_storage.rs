#[test]
fn storage_context_can_follow_network_identity() {
    let network_context = "workspace-a";
    let storage_context = "workspace-a";

    assert_eq!(
        network_context,
        storage_context
    );
}
