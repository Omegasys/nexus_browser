use nexus_browser::engines::engine_api::BrowserEngine;

struct TestEngine {
    initialized: bool,
}

impl TestEngine {
    fn new() -> Self {
        Self {
            initialized: false,
        }
    }
}

impl BrowserEngine for TestEngine {
    fn name(&self) -> &str {
        "test-engine"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn initialize(&mut self) -> Result<(), String> {
        self.initialized = true;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), String> {
        self.initialized = false;
        Ok(())
    }

    fn capabilities(&self) -> Vec<String> {
        vec![
            "rendering".to_string(),
            "sandboxing".to_string(),
        ]
    }
}

#[test]
fn engine_reports_identity() {
    let engine = TestEngine::new();

    assert_eq!(engine.name(), "test-engine");
    assert_eq!(engine.version(), "1.0.0");
}

#[test]
fn engine_can_initialize_and_shutdown() {
    let mut engine = TestEngine::new();

    assert!(!engine.initialized);

    engine.initialize().unwrap();
    assert!(engine.initialized);

    engine.shutdown().unwrap();
    assert!(!engine.initialized);
}

#[test]
fn engine_reports_capabilities() {
    let engine = TestEngine::new();

    let capabilities = engine.capabilities();

    assert!(capabilities.contains(&"rendering".to_string()));
    assert!(capabilities.contains(&"sandboxing".to_string()));
}
