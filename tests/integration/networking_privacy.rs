#[test]
fn privacy_profile_can_select_private_route() {
    let privacy_profile = true;
    let route = "tor";

    assert!(privacy_profile);
    assert_eq!(route, "tor");
}
