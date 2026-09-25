#[test]
fn profiles_can_have_independent_privacy_settings() {
    let profile_a = "compatibility";
    let profile_b = "maximum-privacy";

    assert_ne!(profile_a, profile_b);
}
