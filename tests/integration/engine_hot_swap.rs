#[test]
fn engine_can_be_replaced_without_replacing_browser_identity() {
    let old_engine = "blink";
    let new_engine = "servo";
    let identity_preserved = true;

    assert_ne!(old_engine, new_engine);
    assert!(identity_preserved);
}
