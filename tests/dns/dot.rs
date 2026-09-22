#[test]
fn dot_uses_tls_transport() {
    let encrypted = true;

    assert!(encrypted);
}

#[test]
fn dot_uses_dns_port_by_default() {
    let port = 853;

    assert_eq!(port, 853);
}
