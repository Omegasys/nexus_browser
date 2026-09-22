#[cfg(test)]
mod tests {
    use nexus_browser::microvm::gpu_vm::{
        GpuMode,
        GpuVm,
    };

    #[test]
    fn gpu_vm_can_be_created() {
        let vm = GpuVm::new();

        assert!(!vm.is_running());
    }

    #[test]
    fn software_rendering_can_be_selected() {
        let mut vm = GpuVm::new();

        vm.set_mode(GpuMode::Software)
            .expect("GPU mode should be accepted");

        assert_eq!(
            vm.mode(),
            GpuMode::Software
        );
    }

    #[test]
    fn virtual_gpu_can_be_selected() {
        let mut vm = GpuVm::new();

        vm.set_mode(GpuMode::VirtualGpu)
            .expect("GPU mode should be accepted");

        assert_eq!(
            vm.mode(),
            GpuMode::VirtualGpu
        );
    }

    #[test]
    fn gpu_vm_can_start() {
        let mut vm = GpuVm::new();

        vm.start()
            .expect("GPU VM should start");

        assert!(vm.is_running());
    }
}
