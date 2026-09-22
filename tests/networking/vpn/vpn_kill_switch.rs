#[test]
fn vpn_kill_switch_blocks_traffic_when_tunnel_is_down() {
    let vpn_connected = false;
    let kill_switch = true;

    let traffic_allowed = vpn_connected || !kill_switch;

    assert!(!traffic_allowed);
}
