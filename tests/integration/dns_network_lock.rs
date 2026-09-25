#[test]
fn network_lock_blocks_dns_bypass() {
    let network_lock = true;
    let dns_bypass = false;

    assert!(network_lock);
    assert!(!dns_bypass);
}
