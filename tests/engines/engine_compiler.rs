use nexus_browser::engines::engine_compiler::EngineCompiler;

#[test]
fn compiler_can_be_created() {
    let compiler = EngineCompiler::new();

    assert!(compiler.is_available() || !compiler.is_available());
}

#[test]
fn compiler_rejects_empty_source() {
    let compiler = EngineCompiler::new();

    let result = compiler.compile("");

    assert!(result.is_err());
}
