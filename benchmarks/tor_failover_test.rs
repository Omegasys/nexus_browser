use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TorFailoverStatus {
    Protected,
    CircuitReplaced,
    FailoverBlocked,
    LeakDetected,
}

#[derive(Debug, Clone)]
pub struct TorFailoverTestResult {
    pub status: TorFailoverStatus,
    pub original_circuit: Option<String>,
    pub replacement_circuit: Option<String>,
    pub traffic_blocked_during_transition: bool,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct TorFailoverTest {
    fail_closed: bool,
    allow_direct_fallback: bool,
}

impl TorFailoverTest {
    pub fn new() -> Self {
        Self {
            fail_closed: true,
            allow_direct_fallback: false,
        }
    }

    pub fn fail_closed(mut self, enabled: bool) -> Self {
        self.fail_closed = enabled;
        self
    }

    pub fn allow_direct_fallback(mut self, enabled: bool) -> Self {
        self.allow_direct_fallback = enabled;
        self
    }

    pub fn evaluate(
        &self,
        original_circuit: Option<String>,
        replacement_circuit: Option<String>,
        traffic_blocked_during_transition: bool,
    ) -> TorFailoverTestResult {
        let start = Instant::now();

        let status = if !traffic_blocked_during_transition
            && self.fail_closed
            && !self.allow_direct_fallback
        {
            TorFailoverStatus::LeakDetected
        } else if replacement_circuit.is_some() {
            TorFailoverStatus::CircuitReplaced
        } else if self.fail_closed {
            TorFailoverStatus::FailoverBlocked
        } else {
            TorFailoverStatus::Protected
        };

        TorFailoverTestResult {
            status,
            original_circuit,
            replacement_circuit,
            traffic_blocked_during_transition,
            duration: start.elapsed(),
        }
    }

    pub fn test_transition(
        &self,
        original_circuit: &str,
        replacement_circuit: Option<&str>,
        traffic_blocked: bool,
    ) -> TorFailoverTestResult {
        self.evaluate(
            Some(original_circuit.to_string()),
            replacement_circuit.map(str::to_string),
            traffic_blocked,
        )
    }
}

impl Default for TorFailoverTest {
    fn default() -> Self {
        Self::new()
    }
}
