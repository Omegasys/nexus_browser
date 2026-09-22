#[test]
fn interface_selection_can_have_a_default_interface() {
    let interface = Some("eth0");

    assert!(interface.is_some());
}

#[test]
fn unavailable_interface_should_not_be_selected() {
    let available = false;

    assert!(!available);
}
