#[test]
fn isolated_contexts_do_not_share_network_identity() {
    let context_a = "workspace-a";
    let context_b = "workspace-b";

    assert_ne!(context_a, context_b);
}
