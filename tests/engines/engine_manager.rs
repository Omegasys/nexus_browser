use nexus_browser::engines::engine_manager::EngineManager;

#[test]
fn engine_manager_starts_empty() {
    let manager = EngineManager::new();

    assert_eq!(manager.engine_count(), 0);
}

#[test]
fn engine_manager_can_register_engine() {
    let mut manager = EngineManager::new();

    let result = manager.register_engine("test-engine".to_string());

    assert!(result.is_ok());
    assert_eq!(manager.engine_count(), 1);
}

#[test]
fn engine_manager_can_remove_engine() {
    let mut manager = EngineManager::new();

    manager.register_engine("test-engine".to_string()).unwrap();

    assert!(manager.remove_engine("test-engine"));
    assert_eq!(manager.engine_count(), 0);
}
