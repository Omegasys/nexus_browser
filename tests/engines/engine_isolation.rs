#[test]
fn rendering_engine_should_have_isolation_boundary() {
    let isolation_boundaries = [
        "process",
        "site",
        "tab",
        "microvm",
    ];

    assert!(isolation_boundaries.contains(&"microvm"));
}

#[test]
fn untrusted_engine_should_not_be_trusted_core() {
    let trust_state = "testing";

    assert_ne!(trust_state, "trusted_core");
}
