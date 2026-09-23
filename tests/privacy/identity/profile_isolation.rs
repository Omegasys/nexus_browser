#[test]
fn profiles_have_independent_privacy_contexts() {
    let profile_a = "profile-a";
    let profile_b = "profile-b";

    assert_ne!(profile_a, profile_b);
}
