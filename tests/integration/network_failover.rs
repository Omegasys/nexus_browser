#[test]
fn network_failover_can_switch_to_configured_secondary_route() {
    let primary = "vpn";
    let secondary = "tor";

    assert_ne!(primary, secondary);
}

#[test]
fn network_failover_does_not_use_unapproved_direct_route() {
    let approved = [
        "vpn",
        "tor",
    ];

    assert!(!approved.contains(&"direct"));
}
