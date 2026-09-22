use nexus_browser::engines::engine_tester::EngineTester;

#[test]
fn engine_tester_can_be_created() {
    let tester = EngineTester::new();

    assert!(tester.test_count() >= 0);
}

#[test]
fn tester_can_register_a_test() {
    let mut tester = EngineTester::new();

    tester.add_test("basic-rendering");

    assert!(tester.has_test("basic-rendering"));
}
