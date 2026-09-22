#[cfg(test)]
mod tests {
    use nexus_browser::microvm::microvm_manager::MicroVmManager;

    #[test]
    fn running_vm_can_be_snapshotted() {
        let mut manager = MicroVmManager::new();

        let id = manager.create().expect("VM creation");

        manager.start(&id).expect("VM start");

        let snapshot = manager
            .snapshot(&id)
            .expect("snapshot should succeed");

        assert!(!snapshot.is_empty());
    }

    #[test]
    fn snapshot_has_unique_identifier() {
        let mut manager = MicroVmManager::new();

        let id = manager.create().expect("VM creation");

        manager.start(&id).expect("VM start");

        let first = manager.snapshot(&id).expect("first snapshot");
        let second = manager.snapshot(&id).expect("second snapshot");

        assert_ne!(first, second);
    }
}
