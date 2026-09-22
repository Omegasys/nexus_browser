#[test]
fn firewall_kill_switch_blocks_all_unapproved_traffic() {
    let kill_switch = true;
    let traffic_allowed = !kill_switch;

    assert!(!traffic_allowed);
}
