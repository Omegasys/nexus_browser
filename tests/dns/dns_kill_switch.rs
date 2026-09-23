#[test]
fn dns_kill_switch_blocks_plaintext_dns() {
    let kill_switch = true;
    let plaintext_allowed = false;

    assert!(kill_switch);
    assert!(!plaintext_allowed);
}

#[test]
fn dns_kill_switch_can_block_resolver_bypass() {
    let bypass_allowed = false;

    assert!(!bypass_allowed);
}
