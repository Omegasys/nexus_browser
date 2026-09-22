#[cfg(test)]
mod tests {
    use nexus_browser::microvm::dns_vm::{
        DnsResolver,
        DnsVm,
    };

    #[test]
    fn dns_vm_can_be_created() {
        let vm = DnsVm::new();

        assert!(!vm.is_running());
    }

    #[test]
    fn dns_resolver_can_be_selected() {
        let mut vm = DnsVm::new();

        vm.set_resolver(DnsResolver::DoH)
            .expect("resolver selection should succeed");

        assert_eq!(
            vm.resolver(),
            DnsResolver::DoH
        );
    }

    #[test]
    fn plaintext_dns_can_be_disabled() {
        let mut vm = DnsVm::new();

        vm.set_plaintext_allowed(false);

        assert!(!vm.plaintext_allowed());
    }

    #[test]
    fn dns_vm_can_start() {
        let mut vm = DnsVm::new();

        vm.start()
            .expect("DNS VM should start");

        assert!(vm.is_running());
    }
}
