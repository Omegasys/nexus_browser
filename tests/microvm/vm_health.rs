#[cfg(test)]
mod tests {
    use nexus_browser::microvm::vm_health::VmHealth;

    #[test]
    fn new_vm_is_healthy() {
        let health = VmHealth::new();

        assert!(health.is_healthy());
    }

    #[test]
    fn failed_check_changes_health() {
        let mut health = VmHealth::new();

        health.record_failure(
            "network subsystem",
        );

        assert!(!health.is_healthy());
    }

    #[test]
    fn successful_check_restores_health() {
        let mut health = VmHealth::new();

        health.record_failure(
            "network subsystem",
        );

        health.record_success();

        assert!(health.is_healthy());
    }
}
