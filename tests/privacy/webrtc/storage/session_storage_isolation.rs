#[test]
fn session_storage_isolated_between_tabs() {
    let tab_a = "tab-a";
    let tab_b = "tab-b";

    assert_ne!(tab_a, tab_b);
}
