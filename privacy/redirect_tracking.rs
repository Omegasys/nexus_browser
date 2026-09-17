use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectTrackingMode {
    Disabled,
    Detect,
    Block,
}

#[derive(Debug, Clone)]
pub struct RedirectTrackingProtection {
    pub mode: RedirectTrackingMode,
    blocked_domains: HashSet<String>,
}

impl RedirectTrackingProtection {
    pub fn new() -> Self {
        Self {
            mode: RedirectTrackingMode::Block,
            blocked_domains: HashSet::new(),
        }
    }

    pub fn set_mode(&mut self, mode: RedirectTrackingMode) {
        self.mode = mode;
    }

    pub fn add_domain(&mut self, domain: impl Into<String>) {
        self.blocked_domains
            .insert(domain.into().to_ascii_lowercase());
    }

    pub fn remove_domain(&mut self, domain: &str) {
        self.blocked_domains
            .remove(&domain.to_ascii_lowercase());
    }

    pub fn is_blocked_domain(&self, domain: &str) -> bool {
        self.blocked_domains
            .contains(&domain.to_ascii_lowercase())
    }

    pub fn should_block(&self, domain: &str) -> bool {
        match self.mode {
            RedirectTrackingMode::Disabled => false,
            RedirectTrackingMode::Detect => false,
            RedirectTrackingMode::Block => self.is_blocked_domain(domain),
        }
    }

    pub fn classify_redirect(&self, source: &str, destination: &str) -> bool {
        if source.eq_ignore_ascii_case(destination) {
            return false;
        }

        self.is_blocked_domain(source)
    }
}

impl Default for RedirectTrackingProtection {
    fn default() -> Self {
        Self::new()
    }
}
