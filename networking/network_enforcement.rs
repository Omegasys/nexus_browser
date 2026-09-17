use super::network_policy::NetworkPolicy;
use super::routing_manager::RouteTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementAction {
    Allow,
    Block,
    Redirect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnforcementDecision {
    pub action: EnforcementAction,
    pub target: RouteTarget,
}

#[derive(Debug, Clone)]
pub struct NetworkEnforcement {
    pub enabled: bool,
}

impl NetworkEnforcement {
    pub fn new() -> Self {
        Self {
            enabled: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn evaluate(
        &self,
        target: RouteTarget,
        policy: &NetworkPolicy,
    ) -> EnforcementDecision {
        if !self.enabled {
            return EnforcementDecision {
                action: EnforcementAction::Allow,
                target,
            };
        }

        if policy.allows(target) {
            EnforcementDecision {
                action: EnforcementAction::Allow,
                target,
            }
        } else {
            EnforcementDecision {
                action: EnforcementAction::Block,
                target: RouteTarget::Blocked,
            }
        }
    }
}

impl Default for NetworkEnforcement {
    fn default() -> Self {
        Self::new()
    }
}
