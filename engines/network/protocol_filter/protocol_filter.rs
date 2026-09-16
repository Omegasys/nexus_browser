//! Main protocol filtering engine.

use super::protocol_rule::ProtocolRule;
use super::protocol_registry::ProtocolRegistry;
use super::protocol_policy::ProtocolPolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolFilterState {
    Disabled,
    Active,
    Locked,
}

pub struct ProtocolFilter {
    state: ProtocolFilterState,
    registry: ProtocolRegistry,
    policy: ProtocolPolicy,
    rules: Vec<ProtocolRule>,
}

impl ProtocolFilter {
    pub fn new() -> Self {
        Self {
            state: ProtocolFilterState::Disabled,
            registry: ProtocolRegistry::new(),
            policy: ProtocolPolicy::default(),
            rules: Vec::new(),
        }
    }

    pub fn enable(&mut self) {
        self.state = ProtocolFilterState::Active;
    }

    pub fn disable(&mut self) {
        self.state = ProtocolFilterState::Disabled;
    }

    pub fn lock(&mut self) {
        self.state = ProtocolFilterState::Locked;
    }

    pub fn state(&self) -> ProtocolFilterState {
        self.state
    }

    pub fn active(&self) -> bool {
        matches!(
            self.state,
            ProtocolFilterState::Active
        )
    }

    pub fn add_rule(&mut self, rule: ProtocolRule) {
        self.rules.push(rule);
    }

    pub fn registry(&self) -> &ProtocolRegistry {
        &self.registry
    }

    pub fn policy(&self) -> &ProtocolPolicy {
        &self.policy
    }

    pub fn policy_mut(&mut self) -> &mut ProtocolPolicy {
        &mut self.policy
    }
}

impl Default for ProtocolFilter {
    fn default() -> Self {
        Self::new()
    }
}
