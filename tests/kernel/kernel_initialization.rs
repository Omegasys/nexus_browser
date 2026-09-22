#[cfg(test)]
mod tests {
    use nexus_browser::browser_kernel::kernel_core::KernelCore;

    #[test]
    fn kernel_can_be_created() {
        let kernel = KernelCore::new();

        assert!(kernel.is_ok());
    }

    #[test]
    fn kernel_initial_state_is_valid() {
        let kernel = KernelCore::new()
            .expect("kernel should initialize");

        assert!(!kernel.is_running());
    }

    #[test]
    fn kernel_can_start() {
        let mut kernel = KernelCore::new()
            .expect("kernel should initialize");

        assert!(kernel.start().is_ok());
        assert!(kernel.is_running());
    }

    #[test]
    fn kernel_can_shutdown() {
        let mut kernel = KernelCore::new()
            .expect("kernel should initialize");

        kernel.start()
            .expect("kernel should start");

        assert!(kernel.shutdown().is_ok());
        assert!(!kernel.is_running());
    }

    #[test]
    fn kernel_shutdown_is_safe_when_not_running() {
        let mut kernel = KernelCore::new()
            .expect("kernel should initialize");

        assert!(kernel.shutdown().is_ok());
    }
}
