use nexus_browser::browser_core::networking::dns_manager::DnsMode;

#[test]
fn secure_dns_modes_are_available() {
    let modes = [
        DnsMode::DoH,
        DnsMode::DoT,
        DnsMode::DnsCrypt,
        DnsMode::Tor,
    ];

    assert_eq!(modes.len(), 4);
}

#[test]
fn system_dns_is_distinct_from_encrypted_dns() {
    assert_ne!(DnsMode::System, DnsMode::DoH);
}
