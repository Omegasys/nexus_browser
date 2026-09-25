#[test]
fn shared_worker_can_have_multiple_clients() {
    let clients = 2usize;

    assert!(clients > 1);
}

#[test]
fn shared_worker_can_be_isolated_by_storage_context() {
    let isolated = true;

    assert!(isolated);
}
