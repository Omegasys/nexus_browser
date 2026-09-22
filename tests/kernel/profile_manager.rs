#[cfg(test)]
mod tests {
    use nexus_browser::browser_kernel::profile_manager::ProfileManager;

    #[test]
    fn profile_manager_can_be_created() {
        let manager = ProfileManager::new();

        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn profile_can_be_created() {
        let mut manager = ProfileManager::new();

        manager
            .create("default")
            .expect("profile creation should succeed");

        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn profile_can_be_selected() {
        let mut manager = ProfileManager::new();

        manager
            .create("privacy")
            .expect("profile creation should succeed");

        manager
            .activate("privacy")
            .expect("profile activation should succeed");

        assert_eq!(
            manager.active(),
            Some("privacy")
        );
    }

    #[test]
    fn unknown_profile_cannot_be_activated() {
        let mut manager = ProfileManager::new();

        assert!(
            manager
                .activate("missing")
                .is_err()
        );
    }
}
