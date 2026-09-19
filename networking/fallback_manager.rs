use super::routing_manager::RouteTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FallbackMode {
    Disabled,
    FailClosed,
    Automatic,
    Manual,
}

#[derive(Debug, Clone)]
pub struct FallbackRule {
    pub failed_route: RouteTarget,
    pub fallback_route: RouteTarget,
}

impl FallbackRule {
    pub fn new(
        failed_route: RouteTarget,
        fallback_route: RouteTarget,
    ) -> Self {
        Self {
            failed_route,
            fallback_route,
        }
    }
}

#[derive(Debug)]
pub struct FallbackManager {
    mode: FallbackMode,
    rules: Vec<FallbackRule>,
    last_failure: Option<RouteTarget>,
}

impl Default for FallbackManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FallbackManager {
    pub fn new() -> Self {
        Self {
            mode: FallbackMode::FailClosed,
            rules: Vec::new(),
            last_failure: None,
        }
    }

    pub fn set_mode(&mut self, mode: FallbackMode) {
        self.mode = mode;
    }

    pub fn mode(&self) -> FallbackMode {
        self.mode
    }

    pub fn add_rule(&mut self, rule: FallbackRule) {
        self.rules.push(rule);
    }

    pub fn remove_rules_for(&mut self, route: RouteTarget) {
        self.rules
            .retain(|rule| rule.failed_route != route);
    }

    pub fn record_failure(&mut self, route: RouteTarget) {
        self.last_failure = Some(route);
    }

    pub fn clear_failure(&mut self) {
        self.last_failure = None;
    }

    pub fn fallback_for(&self, route: RouteTarget) -> Option<RouteTarget> {
        if self.mode == FallbackMode::Disabled
            || self.mode == FallbackMode::FailClosed
        {
            return None;
        }

        self.rules
            .iter()
            .find(|rule| rule.failed_route == route)
            .map(|rule| rule.fallback_route)
    }

    pub fn next_route(&self, route: RouteTarget) -> Option<RouteTarget> {
        self.fallback_for(route)
    }

    pub fn should_fail_closed(&self, route: RouteTarget) -> bool {
        match self.mode {
            FallbackMode::Disabled | FallbackMode::FailClosed => true,
            FallbackMode::Automatic | FallbackMode::Manual => {
                self.fallback_for(route).is_none()
            }
        }
    }

    pub fn last_failure(&self) -> Option<RouteTarget> {
        self.last_failure
    }

    pub fn rules(&self) -> &[FallbackRule] {
        &self.rules
    }
}
