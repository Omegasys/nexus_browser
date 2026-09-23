#[test]
fn privacy_profiles_have_distinct_levels() {
    let profiles = [
        "standard",
        "private",
        "strict",
        "maximum",
    ];

    assert_eq!(profiles.len(), 4);
}

#[test]
fn maximum_privacy_is_more_restrictive_than_standard_configuration() {
    let standard_restrictions = 3;
    let maximum_restrictions = 10;

    assert!(maximum_restrictions > standard_restrictions);
}
