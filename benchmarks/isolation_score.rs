use std::collections::HashMap;

/// Isolation properties that can be benchmarked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IsolationComponent {
    ProcessIsolation,
    SiteIsolation,
    TabIsolation,
    StorageIsolation,
    CookieIsolation,
    NetworkIsolation,
    DnsIsolation,
    IdentityIsolation,
    MicroVmIsolation,
    EscapePrevention,
}

/// Result of an isolation benchmark.
#[derive(Debug, Clone)]
pub struct IsolationScoreResult {
    pub score: f64,
    pub components: HashMap<IsolationComponent, f64>,
    pub passed_checks: usize,
    pub failed_checks: usize,
}

impl IsolationScoreResult {
    pub fn new() -> Self {
        Self {
            score: 0.0,
            components: HashMap::new(),
            passed_checks: 0,
            failed_checks: 0,
        }
    }
}

impl Default for IsolationScoreResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Individual isolation test.
#[derive(Debug, Clone)]
pub struct IsolationCheck {
    pub name: String,
    pub component: IsolationComponent,
    pub passed: bool,
    pub weight: f64,
}

/// Isolation benchmark engine.
#[derive(Debug, Clone)]
pub struct IsolationScore {
    checks: Vec<IsolationCheck>,
}

impl IsolationScore {
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
        }
    }

    pub fn add_check(
        &mut self,
        name: impl Into<String>,
        component: IsolationComponent,
        passed: bool,
        weight: f64,
    ) {
        self.checks.push(IsolationCheck {
            name: name.into(),
            component,
            passed,
            weight: weight.max(0.0),
        });
    }

    pub fn run(&self) -> IsolationScoreResult {
        let mut result = IsolationScoreResult::new();

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

        result.passed_checks = self
            .checks
            .iter()
            .filter(|check| check.passed)
            .count();

        result.failed_checks =
            self.checks.len().saturating_sub(result.passed_checks);

        let mut component_values: HashMap<
            IsolationComponent,
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
            let score = if weight == 0.0 {
                0.0
            } else {
                (passed / weight) * 100.0
            };

            result.components.insert(component, score);
        }

        result
    }

    pub fn checks(&self) -> &[IsolationCheck] {
        &self.checks
    }

    pub fn clear(&mut self) {
        self.checks.clear();
    }
}

impl Default for IsolationScore {
    fn default() -> Self {
        Self::new()
    }
}
