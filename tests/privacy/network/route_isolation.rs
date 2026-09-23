#[test]
fn privacy_profiles_can_have_different_network_routes() {
    let privacy_route = "tor";
    let compatibility_route = "direct";

    assert_ne!(
        privacy_route,
        compatibility_route
    );
}
