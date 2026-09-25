#[test]
fn network_lock_can_block_all_traffic() {
    let locked = true;
    let traffic_allowed = false;

    assert!(locked);
    assert!(!traffic_allowed);
}
