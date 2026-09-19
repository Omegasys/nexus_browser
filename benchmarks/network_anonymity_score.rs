use std::collections::HashMap;

/// Components contributing to the network anonymity score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnonymityComponent {
    IpProtection,
    DnsProtection,
    TransportProtection,
    TrafficObfuscation,
    NetworkIsolation,
    WebRtcProtection,
    ProxyProtection,
    RouteConsistency,
    MetadataReduction,
}

/// A single network anonymity check.
#[derive(Debug, Clone)]
pub struct AnonymityCheck {
    pub name: String,
    pub component: AnonymityComponent,
    pub passed: bool,
    pub weight: f64,
}

/// Network anonymity benchmark result.
#[derive(Debug, Clone)]
pub struct NetworkAnonymityResult {
    pub score: f64,
    pub components: HashMap<AnonymityComponent, f64>,
    pub passed_checks: usize,
    pub failed_checks: usize,
    pub total_checks: usize,
}

impl NetworkAnonymityResult {
    pub fn new() -> Self {
        Self {
            score: 0.0,
            components: HashMap::new(),
            passed_checks: 0,
            failed_checks: 0,
            total_checks: 0,
        }
    }
}

impl Default for NetworkAnonymityResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Network anonymity scoring engine.
#[derive(Debug, Clone)]
pub struct NetworkAnonymityScore {
    checks: Vec<AnonymityCheck>,
}

impl NetworkAnonymityScore {
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
        }
    }

    pub fn add_check(
        &mut self,
        name: impl Into<String>,
        component: AnonymityComponent,
        passed: bool,
        weight: f64,
    ) {
        self.checks.push(AnonymityCheck {
            name: name.into(),
            component,
            passed,
            weight: weight.max(0.0),
        });
    }

    pub fn run(&self) -> NetworkAnonymityResult {
        let mut result = NetworkAnonymityResult::new();

        result.total_checks = self.checks.len();

        result.passed_checks = self
            .checks
            .iter()
            .filter(|check| check.passed)
            .count();

        result.failed_checks =
            result.total_checks.saturating_sub(result.passed_checks);

        let total_weight: f64 =
            self.checks.iter().map(|check| check.weight).sum();

        if total_weight == 0.0 {
            return result;
        }

        let passed_weight: f64 = self
            .checks
            .iter()
            .filter(|check| check.passed)
            .map(|check| check.weight)
            .sum();

        result.score = (passed_weight / total_weight) * 100.0;

        let mut component_values: HashMap<
            AnonymityComponent,
            (f64, f64),
        > = HashMap::new();

        for check in &self.checks {
            let entry = component_values
                .entry(check.component)
                .or_insert((0.0, 0.0));

            entry.0 += check.weight;

            if check.passed {
                entry.1 += check.weight;
            }
        }

        for (component, (weight, passed)) in component_values {
            let component_score = if weight == 0.0 {
                0.0
            } else {
                (passed / weight) * 100.0
            };

            result
                .components
                .insert(component, component_score);
        }

        result
    }

    pub fn checks(&self) -> &[AnonymityCheck] {
        &self.checks
    }

    pub fn clear(&mut self) {
        self.checks.clear();
    }
}

impl Default for NetworkAnonymityScore {
    fn default() -> Self {
        Self::new()
    }
}
