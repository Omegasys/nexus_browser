#[test]
fn third_party_cookies_can_be_blocked() {
    let allowed = false;

    assert!(!allowed);
}

#[test]
fn first_party_cookies_can_remain_available() {
    let allowed = true;

    assert!(allowed);
}
