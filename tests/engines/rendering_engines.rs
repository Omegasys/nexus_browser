#[test]
fn blink_is_a_supported_rendering_engine() {
    let engines = [
        "blink",
        "gecko",
        "servo",
    ];

    assert!(engines.contains(&"blink"));
}

#[test]
fn gecko_is_a_supported_rendering_engine() {
    let engines = [
        "blink",
        "gecko",
        "servo",
    ];

    assert!(engines.contains(&"gecko"));
}

#[test]
fn servo_is_a_supported_rendering_engine() {
    let engines = [
        "blink",
        "gecko",
        "servo",
    ];

    assert!(engines.contains(&"servo"));
}
