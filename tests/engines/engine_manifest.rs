use nexus_browser::engines::engine_manifest::EngineManifest;

#[test]
fn manifest_requires_name_and_version() {
    let manifest = EngineManifest::new(
        "test-engine",
        "1.0.0",
    );

    assert_eq!(manifest.name, "test-engine");
    assert_eq!(manifest.version, "1.0.0");
}

#[test]
fn manifest_can_define_capabilities() {
    let mut manifest = EngineManifest::new(
        "test-engine",
        "1.0.0",
    );

    manifest.add_capability("rendering");
    manifest.add_capability("javascript");

    assert!(manifest.capabilities.contains(&"rendering".to_string()));
    assert!(manifest.capabilities.contains(&"javascript".to_string()));
}
