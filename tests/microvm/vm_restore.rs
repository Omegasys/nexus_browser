#[cfg(test)]
mod tests {
    use nexus_browser::microvm::microvm_manager::MicroVmManager;

    #[test]
    fn vm_can_be_restored_from_snapshot() {
        let mut manager = MicroVmManager::new();

        let id = manager.create().expect("VM creation");

        manager.start(&id).expect("VM start");

        let snapshot = manager
            .snapshot(&id)
            .expect("snapshot");

        manager.stop(&id).expect("VM stop");

        assert!(
            manager
                .restore(&id, &snapshot)
                .is_ok()
        );
    }

    #[test]
    fn invalid_snapshot_is_rejected() {
        let mut manager = MicroVmManager::new();

        let id = manager.create().expect("VM creation");

        assert!(
            manager
                .restore(&id, "invalid-snapshot")
                .is_err()
        );
    }
}
