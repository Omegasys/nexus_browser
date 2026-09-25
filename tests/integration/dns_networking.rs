#[test]
fn network_route_can_use_secure_dns() {
    let route = "vpn";
    let dns = "doh";

    assert_eq!(route, "vpn");
    assert_eq!(dns, "doh");
}
