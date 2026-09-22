#[cfg(test)]
mod tests {
    use nexus_browser::microvm::renderer_vm::{
        RendererEngine,
        RendererVm,
    };

    #[test]
    fn renderer_vm_can_be_created() {
        let vm = RendererVm::new();

        assert!(!vm.is_running());
    }

    #[test]
    fn renderer_engine_can_be_selected() {
        let mut vm = RendererVm::new();

        vm.set_engine(RendererEngine::Servo)
            .expect("engine selection should succeed");

        assert_eq!(
            vm.engine(),
            RendererEngine::Servo
        );
    }

    #[test]
    fn renderer_vm_can_start() {
        let mut vm = RendererVm::new();

        vm.set_engine(RendererEngine::Blink)
            .expect("engine selection");

        vm.start()
            .expect("renderer VM should start");

        assert!(vm.is_running());
    }

    #[test]
    fn renderer_vm_can_stop() {
        let mut vm = RendererVm::new();

        vm.start()
            .expect("renderer VM should start");

        vm.stop()
            .expect("renderer VM should stop");

        assert!(!vm.is_running());
    }
}
