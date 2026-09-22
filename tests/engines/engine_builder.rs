use nexus_browser::engines::engine_builder::EngineBuilder;

#[test]
fn builder_can_be_created() {
    let builder = EngineBuilder::new();

    assert!(!builder.target().is_empty());
}

#[test]
fn builder_rejects_missing_source() {
    let builder = EngineBuilder::new();

    let result = builder.build("");

    assert!(result.is_err());
}
