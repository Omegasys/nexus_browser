use nexus_browser::engines::engine_trust::{
    EngineTrust,
    TrustState,
};

#[test]
fn new_engine_is_not_trusted() {
    let trust = EngineTrust::new();

    assert_ne!(trust.state(), TrustState::Trusted);
}

#[test]
fn engine_can_progress_through_trust_states() {
    let mut trust = EngineTrust::new();

    trust.stage();

    assert_eq!(trust.state(), TrustState::Staged);

    trust.begin_testing();

    assert_eq!(trust.state(), TrustState::Testing);
}
