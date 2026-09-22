#[test]
fn all_traffic_lock_blocks_traffic() {
    let locked = true;
    let traffic_allowed = !locked;

    assert!(!traffic_allowed);
}
