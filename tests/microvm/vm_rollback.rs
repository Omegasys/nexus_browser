#[cfg(test)]
mod tests {
    use nexus_browser::microvm::microvm_manager::MicroVmManager;

    #[test]
    fn vm_can_rollback() {
        let mut manager = MicroVmManager::new();

        let id = manager.create().expect("VM creation");

        manager.start(&id).expect("VM start");

        manager
            .snapshot(&id)
            .expect("snapshot");

        assert!(
            manager
                .rollback(&id)
                .is_ok()
        );
    }

    #[test]
    fn rollback_unknown_vm_fails() {
        let mut manager = MicroVmManager::new();

        assert!(
            manager
                .rollback("missing")
                .is_err()
        );
    }
}
