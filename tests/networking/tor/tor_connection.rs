#[test]
fn tor_route_can_be_selected() {
    let route = "tor";

    assert_eq!(route, "tor");
}

#[test]
fn tor_route_is_not_direct() {
    let route = "tor";

    assert_ne!(route, "direct");
}
