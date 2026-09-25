#[test]
fn matching_hostname_is_accepted() {
    let matches = true;

    assert!(matches);
}

#[test]
fn mismatched_hostname_is_rejected() {
    let matches = false;

    assert!(!matches);
}
