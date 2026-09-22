#[cfg(test)]
mod tests {
    use nexus_browser::microvm::storage_vm::{
        StorageMode,
        StorageVm,
    };

    #[test]
    fn storage_vm_can_be_created() {
        let vm = StorageVm::new();

        assert!(!vm.is_running());
    }

    #[test]
    fn temporary_storage_can_be_selected() {
        let mut vm = StorageVm::new();

        vm.set_mode(StorageMode::Temporary)
            .expect("storage mode should be accepted");

        assert_eq!(
            vm.mode(),
            StorageMode::Temporary
        );
    }

    #[test]
    fn disposable_storage_can_be_selected() {
        let mut vm = StorageVm::new();

        vm.set_mode(StorageMode::Disposable)
            .expect("storage mode should be accepted");

        assert_eq!(
            vm.mode(),
            StorageMode::Disposable
        );
    }

    #[test]
    fn storage_vm_can_start() {
        let mut vm = StorageVm::new();

        vm.start()
            .expect("storage VM should start");

        assert!(vm.is_running());
    }
}
