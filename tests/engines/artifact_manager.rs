use nexus_browser::engines::artifact_manager::ArtifactManager;

#[test]
fn artifact_manager_starts_empty() {
    let manager = ArtifactManager::new();

    assert_eq!(manager.artifact_count(), 0);
}

#[test]
fn artifact_manager_tracks_artifacts() {
    let mut manager = ArtifactManager::new();

    manager.register(
        "blink",
        "1.0.0",
        "/engines/blink.wasm",
    );

    assert_eq!(manager.artifact_count(), 1);
    assert!(manager.contains("blink"));
}
