#[cfg(test)]
mod tests {
    use nexus_browser::browser_kernel::resource_manager::ResourceManager;

    #[test]
    fn resource_manager_starts_empty() {
        let manager = ResourceManager::new();

        assert_eq!(manager.resource_count(), 0);
    }

    #[test]
    fn resource_can_be_registered() {
        let mut manager = ResourceManager::new();

        manager
            .register("renderer", 1024)
            .expect("resource registration should succeed");

        assert_eq!(manager.resource_count(), 1);
    }

    #[test]
    fn resource_usage_can_be_updated() {
        let mut manager = ResourceManager::new();

        manager
            .register("renderer", 1024)
            .expect("registration should succeed");

        manager
            .set_usage("renderer", 512)
            .expect("usage update should succeed");

        assert_eq!(
            manager.usage("renderer"),
            Some(512)
        );
    }

    #[test]
    fn resource_cannot_exceed_limit() {
        let mut manager = ResourceManager::new();

        manager
            .register("renderer", 1024)
            .expect("registration should succeed");

        assert!(
            manager
                .set_usage("renderer", 2048)
                .is_err()
        );
    }
}
