#[cfg(test)]
mod tests {
    use nexus_browser::microvm::microvm_manager::MicroVmManager;

    #[test]
    fn vm_can_be_started() {
        let mut manager = MicroVmManager::new();

        let id = manager.create().expect("VM creation");

        manager
            .start(&id)
            .expect("VM should start");

        assert!(manager.is_running(&id));
    }

    #[test]
    fn vm_can_be_stopped() {
        let mut manager = MicroVmManager::new();

        let id = manager.create().expect("VM creation");

        manager.start(&id).expect("VM start");
        manager.stop(&id).expect("VM stop");

        assert!(!manager.is_running(&id));
    }

    #[test]
    fn unknown_vm_cannot_be_started() {
        let mut manager = MicroVmManager::new();

        assert!(
            manager
                .start("missing")
                .is_err()
        );
    }

    #[test]
    fn stopped_vm_can_be_started_again() {
        let mut manager = MicroVmManager::new();

        let id = manager.create().expect("VM creation");

        manager.start(&id).expect("first start");
        manager.stop(&id).expect("stop");
        manager.start(&id).expect("second start");

        assert!(manager.is_running(&id));
    }
}
