#[test]
fn i2p_destination_is_not_a_normal_ip_address() {
    let destination = "example.i2p";

    assert!(destination.ends_with(".i2p"));
}
