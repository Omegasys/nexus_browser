#[cfg(test)]
mod tests {
    use nexus_browser::browser_kernel::engine_registry::EngineRegistry;

    #[test]
    fn registry_can_be_created() {
        let registry = EngineRegistry::new();

        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn engine_can_be_registered() {
        let mut registry = EngineRegistry::new();

        let result = registry.register(
            "test-engine".to_string(),
            "1.0.0".to_string(),
        );

        assert!(result.is_ok());
        assert_eq!(registry.count(), 1);
    }

    #[test]
    fn registered_engine_can_be_found() {
        let mut registry = EngineRegistry::new();

        registry
            .register(
                "blink".to_string(),
                "1.0.0".to_string(),
            )
            .expect("registration should succeed");

        let engine = registry.get("blink");

        assert!(engine.is_some());
    }

    #[test]
    fn unknown_engine_is_not_found() {
        let registry = EngineRegistry::new();

        assert!(registry.get("missing").is_none());
    }

    #[test]
    fn duplicate_engine_registration_is_rejected() {
        let mut registry = EngineRegistry::new();

        registry
            .register(
                "blink".to_string(),
                "1.0.0".to_string(),
            )
            .expect("first registration should succeed");

        assert!(
            registry
                .register(
                    "blink".to_string(),
                    "2.0.0".to_string(),
                )
                .is_err()
        );
    }
}
