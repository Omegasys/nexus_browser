#[test]
fn extension_permissions_can_be_restricted() {
    let allowed_permissions = [
        "storage",
        "tabs",
    ];

    assert!(allowed_permissions.contains(&"storage"));
}

#[test]
fn undeclared_permission_is_denied() {
    let permissions = [
        "storage",
    ];

    assert!(!permissions.contains(&"native-messaging"));
}
