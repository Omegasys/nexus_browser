use nexus_browser::engines::engine_sandbox::EngineSandbox;

#[test]
fn sandbox_defaults_to_restricted_mode() {
    let sandbox = EngineSandbox::new();

    assert!(sandbox.is_restricted());
}

#[test]
fn sandbox_blocks_unapproved_network_access() {
    let sandbox = EngineSandbox::new();

    assert!(!sandbox.network_access_allowed("unknown"));
}

#[test]
fn sandbox_allows_explicit_capability() {
    let mut sandbox = EngineSandbox::new();

    sandbox.allow_capability("rendering");

    assert!(sandbox.capability_allowed("rendering"));
}
