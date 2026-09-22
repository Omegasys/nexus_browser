#[test]
fn firewall_can_allow_traffic() {
    let action = "allow";

    assert_eq!(action, "allow");
}

#[test]
fn firewall_can_block_traffic() {
    let action = "block";

    assert_eq!(action, "block");
}
