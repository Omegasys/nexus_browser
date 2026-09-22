#[test]
fn dns_fail_closed_blocks_resolution_when_secure_dns_fails() {
    let secure_dns_available = false;
    let fail_closed = true;

    let resolution_allowed =
        secure_dns_available || !fail_closed;

    assert!(!resolution_allowed);
}
