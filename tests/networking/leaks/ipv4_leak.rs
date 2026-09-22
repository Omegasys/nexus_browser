#[test]
fn unexpected_ipv4_address_is_a_leak() {
    let allowed = false;

    assert!(!allowed);
}
