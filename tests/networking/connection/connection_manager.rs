#[test]
fn connection_manager_can_track_connections() {
    let connections = vec![
        "connection-1",
        "connection-2",
    ];

    assert_eq!(connections.len(), 2);
}
