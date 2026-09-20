use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VpnFailoverStatus {
    Protected,
    FailoverSuccessful,
    FailoverFailed,
    LeakDetected,
}

#[derive(Debug, Clone)]
pub struct VpnFailoverTestResult {
    pub status: VpnFailoverStatus,
    pub original_route: Option<String>,
    pub fallback_route: Option<String>,
    pub traffic_blocked_during_transition: bool,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct VpnFailoverTest {
    fail_closed: bool,
    require_fallback: bool,
}

impl VpnFailoverTest {
    pub fn new() -> Self {
        Self {
            fail_closed: true,
            require_fallback: false,
        }
    }

    pub fn fail_closed(mut self, enabled: bool) -> Self {
        self.fail_closed = enabled;
        self
    }

    pub fn require_fallback(mut self, enabled: bool) -> Self {
        self.require_fallback = enabled;
        self
    }

    pub fn evaluate(
        &self,
        original_route: Option<String>,
        fallback_route: Option<String>,
        traffic_blocked_during_transition: bool,
    ) -> VpnFailoverTestResult {
        let start = Instant::now();

        let status = if !traffic_blocked_during_transition && self.fail_closed {
            VpnFailoverStatus::LeakDetected
        } else if fallback_route.is_some() {
            VpnFailoverStatus::FailoverSuccessful
        } else if self.require_fallback {
            VpnFailoverStatus::FailoverFailed
        } else {
            VpnFailoverStatus::Protected
        };

        VpnFailoverTestResult {
            status,
            original_route,
            fallback_route,
            traffic_blocked_during_transition,
            duration: start.elapsed(),
        }
    }

    pub fn test_transition(
        &self,
        original_route: &str,
        fallback_route: Option<&str>,
        traffic_blocked: bool,
    ) -> VpnFailoverTestResult {
        self.evaluate(
            Some(original_route.to_string()),
            fallback_route.map(str::to_string),
            traffic_blocked,
        )
    }
}

impl Default for VpnFailoverTest {
    fn default() -> Self {
        Self::new()
    }
}
