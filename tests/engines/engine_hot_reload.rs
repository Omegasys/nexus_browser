use nexus_browser::engines::engine_hot_reload::EngineHotReload;

#[test]
fn hot_reload_can_be_enabled() {
    let mut reload = EngineHotReload::new();

    assert!(!reload.enabled());

    reload.enable();

    assert!(reload.enabled());
}

#[test]
fn hot_reload_can_be_disabled() {
    let mut reload = EngineHotReload::new();

    reload.enable();
    reload.disable();

    assert!(!reload.enabled());
}
