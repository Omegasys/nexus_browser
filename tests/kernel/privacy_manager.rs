#[cfg(test)]
mod tests {
    use nexus_browser::browser_kernel::privacy_manager::PrivacyManager;

    #[test]
    fn privacy_manager_can_be_created() {
        let manager = PrivacyManager::new();

        assert!(manager.is_enabled());
    }

    #[test]
    fn privacy_level_can_be_changed() {
        let mut manager = PrivacyManager::new();

        manager
            .set_level("maximum")
            .expect("privacy level should be accepted");

        assert_eq!(
            manager.level(),
            "maximum"
        );
    }

    #[test]
    fn invalid_privacy_level_is_rejected() {
        let mut manager = PrivacyManager::new();

        assert!(
            manager
                .set_level("invalid")
                .is_err()
        );
    }

    #[test]
    fn privacy_manager_can_be_reset() {
        let mut manager = PrivacyManager::new();

        manager
            .set_level("maximum")
            .expect("level should change");

        manager.reset();

        assert_eq!(
            manager.level(),
            "standard"
        );
    }
}
