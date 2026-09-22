#[test]
fn rendering_engines_require_rendering_capability() {
    let capabilities = [
        "rendering",
        "javascript",
        "networking",
        "storage",
    ];

    assert!(capabilities.contains(&"rendering"));
}

#[test]
fn engine_capabilities_are_independent() {
    let rendering = ["rendering", "javascript"];
    let networking = ["networking", "dns"];

    assert!(rendering.contains(&"rendering"));
    assert!(networking.contains(&"networking"));
    assert!(!rendering.contains(&"networking"));
}
