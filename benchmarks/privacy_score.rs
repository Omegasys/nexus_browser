use std::collections::HashMap;

/// A category contributing to the overall privacy score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrivacyScoreCategory {
    NetworkPrivacy,
    DnsPrivacy,
    StoragePrivacy,
    CookiePrivacy,
    FingerprintResistance,
    WebRtcPrivacy,
    TrackingProtection,
    Isolation,
}

/// A normalized privacy benchmark result.
///
/// Scores range from 0.0 to 100.0.
#[derive(Debug, Clone)]
pub struct PrivacyScoreResult {
    pub score: f64,
    pub categories: HashMap<PrivacyScoreCategory, f64>,
    pub passed_checks: usize,
    pub failed_checks: usize,
    pub total_checks: usize,
}

impl PrivacyScoreResult {
    pub fn new() -> Self {
        Self {
            score: 0.0,
            categories: HashMap::new(),
            passed_checks: 0,
            failed_checks: 0,
            total_checks: 0,
        }
    }

    pub fn percentage(&self) -> f64 {
        self.score
    }
}

impl Default for PrivacyScoreResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Individual privacy test.
#[derive(Debug, Clone)]
pub struct PrivacyCheck {
    pub name: String,
    pub category: PrivacyScoreCategory,
    pub passed: bool,
    pub weight: f64,
}

/// Privacy benchmark engine.
#[derive(Debug, Clone)]
pub struct PrivacyScore {
    checks: Vec<PrivacyCheck>,
}

impl PrivacyScore {
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
        }
    }

    pub fn add_check(
        &mut self,
        name: impl Into<String>,
        category: PrivacyScoreCategory,
        passed: bool,
        weight: f64,
    ) {
        self.checks.push(PrivacyCheck {
            name: name.into(),
            category,
            passed,
            weight: weight.max(0.0),
        });
    }

    pub fn run(&self) -> PrivacyScoreResult {
        let mut result = PrivacyScoreResult::new();

        let total_weight: f64 = self.checks.iter().map(|check| check.weight).sum();

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

        result.total_checks = self.checks.len();
        result.passed_checks = self
            .checks
            .iter()
            .filter(|check| check.passed)
            .count();

        result.failed_checks =
            result.total_checks.saturating_sub(result.passed_checks);

        let mut category_weights: HashMap<PrivacyScoreCategory, (f64, f64)> =
            HashMap::new();

        for check in &self.checks {
            let entry = category_weights
                .entry(check.category)
                .or_insert((0.0, 0.0));

            entry.0 += check.weight;

            if check.passed {
                entry.1 += check.weight;
            }
        }

        for (category, (weight, passed)) in category_weights {
            let score = if weight == 0.0 {
                0.0
            } else {
                (passed / weight) * 100.0
            };

            result.categories.insert(category, score);
        }

        result
    }

    pub fn checks(&self) -> &[PrivacyCheck] {
        &self.checks
    }

    pub fn clear(&mut self) {
        self.checks.clear();
    }
}

impl Default for PrivacyScore {
    fn default() -> Self {
        Self::new()
    }
}
