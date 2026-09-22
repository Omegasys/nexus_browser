use nexus_browser::engines::engine_health::{
    EngineHealth,
    HealthState,
};

#[test]
fn new_engine_is_healthy_or_unknown() {
    let health = EngineHealth::new();

    assert!(
        matches!(
            health.state(),
            HealthState::Unknown | HealthState::Healthy
        )
    );
}

#[test]
fn failed_engine_is_detected() {
    let mut health = EngineHealth::new();

    health.record_failure();

    assert!(health.failure_count() >= 1);
}
