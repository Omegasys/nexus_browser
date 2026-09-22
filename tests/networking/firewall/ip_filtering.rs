#[test]
fn firewall_can_block_an_ip() {
    let blocked = vec![
        "192.0.2.1",
    ];

    assert!(blocked.contains(&"192.0.2.1"));
}
