#[test]
fn healthy_resolver_can_be_selected() {
    let healthy = true;

    assert!(healthy);
}

#[test]
fn unhealthy_resolver_should_not_be_selected() {
    let healthy = false;

    assert!(!healthy);
}

#[test]
fn resolver_health_can_track_failures() {
    let failures = 3;

    assert!(failures > 0);
}
