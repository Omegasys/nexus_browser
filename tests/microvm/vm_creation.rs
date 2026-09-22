#[cfg(test)]
mod tests {
    use nexus_browser::microvm::microvm_manager::MicroVmManager;

    #[test]
    fn manager_starts_empty() {
        let manager = MicroVmManager::new();

        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn vm_can_be_created() {
        let mut manager = MicroVmManager::new();

        let id = manager
            .create()
            .expect("VM creation should succeed");

        assert!(manager.get(&id).is_some());
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn multiple_vms_can_be_created() {
        let mut manager = MicroVmManager::new();

        manager.create().expect("first VM");
        manager.create().expect("second VM");
        manager.create().expect("third VM");

        assert_eq!(manager.count(), 3);
    }

    #[test]
    fn vm_ids_are_unique() {
        let mut manager = MicroVmManager::new();

        let first = manager.create().expect("first VM");
        let second = manager.create().expect("second VM");

        assert_ne!(first, second);
    }
}
