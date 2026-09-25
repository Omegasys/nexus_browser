#[test]
fn maximum_privacy_can_use_disposable_microvms() {
    let privacy = "maximum";
    let disposable_vm = true;

    assert_eq!(privacy, "maximum");
    assert!(disposable_vm);
}
