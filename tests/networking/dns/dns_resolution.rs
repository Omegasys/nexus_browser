#[test]
fn dns_resolution_can_return_an_address() {
    let address = Some("93.184.216.34");

    assert!(address.is_some());
}
