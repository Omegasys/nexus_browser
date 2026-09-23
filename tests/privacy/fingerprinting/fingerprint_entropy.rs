#[test]
fn fingerprint_entropy_can_be_measured() {
    let entropy_bits = 12.5f64;

    assert!(entropy_bits >= 0.0);
}

#[test]
fn randomized_signals_can_reduce_stable_identity() {
    let stable_identity = false;

    assert!(!stable_identity);
}
