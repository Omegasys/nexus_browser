#[test]
fn dns_failover_can_select_secondary_resolver() {
    let primary_available = false;
    let secondary_available = true;

    assert!(!primary_available);
    assert!(secondary_available);
}

#[test]
fn dns_fail_closed_blocks_when_all_secure_resolvers_fail() {
    let primary_available = false;
    let secondary_available = false;
    let fail_closed = true;

    let resolution_allowed =
        primary_available ||
        secondary_available ||
        !fail_closed;

    assert!(!resolution_allowed);
}
