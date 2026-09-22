#[cfg(test)]
mod tests {
    use nexus_browser::browser_kernel::security_manager::SecurityManager;

    #[test]
    fn security_manager_starts_enabled() {
        let manager = SecurityManager::new();

        assert!(manager.is_enabled());
    }

    #[test]
    fn security_policy_can_be_enabled() {
        let mut manager = SecurityManager::new();

        manager
            .enable()
            .expect("security manager should enable");

        assert!(manager.is_enabled());
    }

    #[test]
    fn security_policy_can_be_disabled() {
        let mut manager = SecurityManager::new();

        manager
            .disable()
            .expect("security manager should disable");

        assert!(!manager.is_enabled());
    }

    #[test]
    fn security_violation_can_be_recorded() {
        let mut manager = SecurityManager::new();

        manager.record_violation(
            "test violation".to_string()
        );

        assert_eq!(
            manager.violation_count(),
            1
        );
    }
}
