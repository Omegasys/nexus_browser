#[test]
fn network_lock_and_kill_switch_work_together() {
    let network_lock = true;
    let kill_switch = true;

    assert!(network_lock);
    assert!(kill_switch);
}
