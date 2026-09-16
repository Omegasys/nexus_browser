// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackingCategory {
    Advertising,
    Analytics,
    Social,
    Cryptomining,
    Fingerprinting,
    Telemetry,
    KnownTracker,
}

pub struct TrackingProtection {
    enabled: bool,
    blocked_categories: HashSet<TrackingCategory>,
    blocked_domains: HashSet<String>,
}

impl TrackingProtection {
    pub fn new() -> Self {
        let mut blocked_categories = HashSet::new();

        blocked_categories.insert(TrackingCategory::Advertising);
        blocked_categories.insert(TrackingCategory::Analytics);
        blocked_categories.insert(TrackingCategory::Social);
        blocked_categories.insert(TrackingCategory::Cryptomining);
        blocked_categories.insert(TrackingCategory::Fingerprinting);
        blocked_categories.insert(TrackingCategory::KnownTracker);

        Self {
            enabled: true,
            blocked_categories,
            blocked_domains: HashSet::new(),
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn block_category(&mut self, category: TrackingCategory) {
        self.blocked_categories.insert(category);
    }

    pub fn allow_category(&mut self, category: &TrackingCategory) {
        self.blocked_categories.remove(category);
    }

    pub fn block_domain(&mut self, domain: impl Into<String>) {
        self.blocked_domains.insert(domain.into());
    }

    pub fn allow_domain(&mut self, domain: &str) {
        self.blocked_domains.remove(domain);
    }

    pub fn is_domain_blocked(&self, domain: &str) -> bool {
        self.enabled && self.blocked_domains.contains(domain)
    }

    pub fn is_category_blocked(&self, category: &TrackingCategory) -> bool {
        self.enabled && self.blocked_categories.contains(category)
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for TrackingProtection {
    fn default() -> Self {
        Self::new()
    }
}
