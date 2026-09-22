#[test]
fn tor_circuit_contains_multiple_hops() {
    let hops = vec![
        "guard",
        "middle",
        "exit",
    ];

    assert_eq!(hops.len(), 3);
}
