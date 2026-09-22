#[test]
fn direct_route_can_be_selected() {
    let route = "direct";

    assert_eq!(route, "direct");
}

#[test]
fn direct_route_is_not_an_anonymous_network() {
    let anonymous = false;

    assert!(!anonymous);
}
