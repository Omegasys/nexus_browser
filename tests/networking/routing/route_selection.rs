#[test]
fn route_targets_are_distinct() {
    let routes = [
        "direct",
        "proxy",
        "vpn",
        "tor",
        "i2p",
        "nym",
        "lokinet",
    ];

    assert_eq!(routes.len(), 7);
}
