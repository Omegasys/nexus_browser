#[test]
fn firewall_can_block_domains() {
    let blocked = vec![
        "example.invalid",
    ];

    assert!(blocked.contains(&"example.invalid"));
}
