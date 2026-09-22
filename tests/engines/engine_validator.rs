use nexus_browser::engines::engine_validator::EngineValidator;

#[test]
fn validator_can_be_created() {
    let validator = EngineValidator::new();

    assert!(validator.is_ready());
}

#[test]
fn validator_rejects_empty_artifact() {
    let validator = EngineValidator::new();

    let result = validator.validate(&[]);

    assert!(result.is_err());
}
