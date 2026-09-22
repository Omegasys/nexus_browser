#[cfg(test)]
mod tests {
    use nexus_browser::microvm::microvm_manager::MicroVmManager;

    #[test]
    fn failed_vm_can_attempt_recovery() {
        let mut manager = MicroVmManager::new();

        let id = manager.create().expect("VM creation");

        manager
            .mark_failed(&id)
            .expect("VM should be marked failed");

        assert!(
            manager
                .recover(&id)
                .is_ok()
        );
    }

    #[test]
    fn recovery_of_unknown_vm_fails() {
        let mut manager = MicroVmManager::new();

        assert!(
            manager
                .recover("missing")
                .is_err()
        );
    }
}
