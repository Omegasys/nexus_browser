#[test]
fn proxy_chain_can_contain_multiple_hops() {
    let chain = vec![
        "proxy-a",
        "proxy-b",
    ];

    assert_eq!(chain.len(), 2);
}
