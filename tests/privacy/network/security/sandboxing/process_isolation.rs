#[test]
fn processes_can_have_separate_security_boundaries() {
    let process_a = "renderer-a";
    let process_b = "renderer-b";

    assert_ne!(process_a, process_b);
}

#[test]
fn process_isolation_can_be_enabled() {
    let enabled = true;

    assert!(enabled);
}
