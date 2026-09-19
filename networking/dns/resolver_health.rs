use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::resolver_registry::{ResolverId, ResolverRegistry, ResolverState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthState {
    Unknown,
    Healthy,
    Degraded,
    Unhealthy,
    Quarantined,
}

#[derive(Debug, Clone)]
pub struct ResolverHealth {
    pub resolver: ResolverId,
    pub state: HealthState,
    pub consecutive_failures: u32,
    pub consecutive_successes: u32,
    pub latency: Option<Duration>,
    pub last_check: Option<Instant>,
    pub last_error: Option<String>,
}

impl ResolverHealth {
    pub fn new(resolver: ResolverId) -> Self {
        Self {
            resolver,
            state: HealthState::Unknown,
            consecutive_failures: 0,
            consecutive_successes: 0,
            latency: None,
            last_check: None,
            last_error: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct ResolverHealthMonitor {
    health: HashMap<ResolverId, ResolverHealth>,
    failure_threshold: u32,
    success_threshold: u32,
}

impl ResolverHealthMonitor {
    pub fn new() -> Self {
        Self {
            health: HashMap::new(),
            failure_threshold: 3,
            success_threshold: 2,
        }
    }

    pub fn register(&mut self, resolver: ResolverId) {
        self.health
            .entry(resolver)
            .or_insert_with(|| ResolverHealth::new(resolver));
    }

    pub fn report_success(
        &mut self,
        resolver: ResolverId,
        latency: Duration,
    ) {
        let health = self
            .health
            .entry(resolver)
            .or_insert_with(|| ResolverHealth::new(resolver));

        health.consecutive_successes =
            health.consecutive_successes.saturating_add(1);

        health.consecutive_failures = 0;
        health.latency = Some(latency);
        health.last_check = Some(Instant::now());
        health.last_error = None;

        if health.consecutive_successes >= self.success_threshold {
            health.state = HealthState::Healthy;
        } else {
            health.state = HealthState::Degraded;
        }
    }

    pub fn report_failure(
        &mut self,
        resolver: ResolverId,
        error: impl Into<String>,
    ) {
        let health = self
            .health
            .entry(resolver)
            .or_insert_with(|| ResolverHealth::new(resolver));

        health.consecutive_failures =
            health.consecutive_failures.saturating_add(1);

        health.consecutive_successes = 0;
        health.last_check = Some(Instant::now());
        health.last_error = Some(error.into());

        if health.consecutive_failures >= self.failure_threshold {
            health.state = HealthState::Unhealthy;
        } else {
            health.state = HealthState::Degraded;
        }
    }

    pub fn quarantine(&mut self, resolver: ResolverId) {
        if let Some(health) = self.health.get_mut(&resolver) {
            health.state = HealthState::Quarantined;
        }
    }

    pub fn restore(&mut self, resolver: ResolverId) {
        if let Some(health) = self.health.get_mut(&resolver) {
            health.state = HealthState::Unknown;
            health.consecutive_failures = 0;
            health.consecutive_successes = 0;
        }
    }

    pub fn is_healthy(&self, resolver: ResolverId) -> bool {
        self.health
            .get(&resolver)
            .map(|health| health.state == HealthState::Healthy)
            .unwrap_or(false)
    }

    pub fn synchronize_registry(&self, registry: &mut ResolverRegistry) {
        for (resolver, health) in &self.health {
            let state = match health.state {
                HealthState::Healthy | HealthState::Degraded => {
                    ResolverState::Available
                }
                HealthState::Unhealthy => ResolverState::Failed,
                HealthState::Quarantined => ResolverState::Quarantined,
                HealthState::Unknown => ResolverState::Available,
            };

            registry.set_state(*resolver, state);
        }
    }

    pub fn get(&self, resolver: ResolverId) -> Option<&ResolverHealth> {
        self.health.get(&resolver)
    }

    pub fn all(&self) -> &HashMap<ResolverId, ResolverHealth> {
        &self.health
    }
}
