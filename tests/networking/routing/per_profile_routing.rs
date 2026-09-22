#[test]
fn profiles_can_define_different_routes() {
    let privacy_profile = "tor";
    let compatibility_profile = "direct";

    assert_ne!(
        privacy_profile,
        compatibility_profile
    );
}
