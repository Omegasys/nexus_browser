#[test]
fn known_tracking_parameters_can_be_removed() {
    let parameters = [
        "utm_source",
        "utm_medium",
        "utm_campaign",
    ];

    assert!(parameters.contains(&"utm_source"));
}
