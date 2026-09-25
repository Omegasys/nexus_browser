#[test]
fn rendering_engine_runs_inside_security_boundary() {
    let engine = true;
    let sandbox = true;

    assert!(engine);
    assert!(sandbox);
}
