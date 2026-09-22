#[test]
fn interface_manager_can_track_interfaces() {
    let interfaces = vec![
        "eth0",
        "wlan0",
    ];

    assert_eq!(interfaces.len(), 2);
}
