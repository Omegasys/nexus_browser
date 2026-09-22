use nexus_browser::browser_core::networking::dns_manager::DnsManager;

#[test]
fn dns_manager_can_be_created() {
    let manager = DnsManager::new();

    assert!(!manager.is_running());
}

#[test]
fn dns_manager_can_start() {
    let mut manager = DnsManager::new();

    manager.start().unwrap();

    assert!(manager.is_running());
}

#[test]
fn dns_manager_can_stop() {
    let mut manager = DnsManager::new();

    manager.start().unwrap();
    manager.stop().unwrap();

    assert!(!manager.is_running());
}
