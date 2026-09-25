#[test]
fn cookie_can_be_stored() {
    let mut cookies = Vec::new();

    cookies.push("session");

    assert_eq!(cookies.len(), 1);
}

#[test]
fn cookie_can_be_removed() {
    let mut cookies = vec![
        "session",
        "preference",
    ];

    cookies.retain(|cookie| *cookie != "session");

    assert!(!cookies.contains(&"session"));
}
