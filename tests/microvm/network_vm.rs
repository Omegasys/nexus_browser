#[cfg(test)]
mod tests {
    use nexus_browser::microvm::network_vm::{
        NetworkRoute,
        NetworkVm,
    };

    #[test]
    fn network_vm_defaults_to_direct() {
        let vm = NetworkVm::new();

        assert_eq!(
            vm.route(),
            NetworkRoute::Direct
        );
    }

    #[test]
    fn network_route_can_be_changed() {
        let mut vm = NetworkVm::new();

        vm.set_route(NetworkRoute::Tor)
            .expect("route selection should succeed");

        assert_eq!(
            vm.route(),
            NetworkRoute::Tor
        );
    }

    #[test]
    fn network_vm_can_start() {
        let mut vm = NetworkVm::new();

        vm.start()
            .expect("network VM should start");

        assert!(vm.is_running());
    }

    #[test]
    fn direct_fallback_can_be_disabled() {
        let mut vm = NetworkVm::new();

        vm.set_fail_closed(true);

        assert!(vm.fail_closed());
    }
}
