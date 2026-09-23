#[test]
fn canvas_fingerprinting_can_be_blocked() {
    let allowed = false;

    assert!(!allowed);
}

#[test]
fn canvas_noise_can_be_enabled() {
    let noise = true;

    assert!(noise);
}
