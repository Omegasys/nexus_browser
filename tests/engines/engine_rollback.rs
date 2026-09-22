use nexus_browser::engines::engine_rollback::EngineRollback;

#[test]
fn rollback_manager_starts_empty() {
    let rollback = EngineRollback::new();

    assert_eq!(rollback.version_count(), 0);
}

#[test]
fn rollback_manager_tracks_versions() {
    let mut rollback = EngineRollback::new();

    rollback.record_version("1.0.0");
    rollback.record_version("1.1.0");

    assert_eq!(rollback.version_count(), 2);
}

#[test]
fn rollback_returns_previous_version() {
    let mut rollback = EngineRollback::new();

    rollback.record_version("1.0.0");
    rollback.record_version("1.1.0");

    assert_eq!(
        rollback.previous_version(),
        Some("1.0.0")
    );
}
