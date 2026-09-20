use std::collections::HashSet;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NetworkProtocol {
    Tcp,
    Udp,
    Quic,
    Icmp,
    Icmpv6,
    Http,
    Https,
    Http3,
    Dns,
    WebRtc,
    Torrent,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolLeakStatus {
    Protected,
    LeakDetected,
    Blocked,
    FailedClosed,
}

#[derive(Debug, Clone)]
pub struct ProtocolLeakTestResult {
    pub status: ProtocolLeakStatus,
    pub observed: Vec<NetworkProtocol>,
    pub unexpected: Vec<NetworkProtocol>,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct ProtocolLeakTest {
    allowed: HashSet<NetworkProtocol>,
    blocked: HashSet<NetworkProtocol>,
    fail_closed: bool,
}

impl ProtocolLeakTest {
    pub fn new() -> Self {
        Self {
            allowed: HashSet::new(),
            blocked: HashSet::new(),
            fail_closed: true,
        }
    }

    pub fn allow(mut self, protocol: NetworkProtocol) -> Self {
        self.allowed.insert(protocol);
        self
    }

    pub fn block(mut self, protocol: NetworkProtocol) -> Self {
        self.blocked.insert(protocol);
        self
    }

    pub fn fail_closed(mut self, enabled: bool) -> Self {
        self.fail_closed = enabled;
        self
    }

    pub fn evaluate(
        &self,
        observed: &[NetworkProtocol],
    ) -> ProtocolLeakTestResult {
        let start = Instant::now();

        let unexpected: Vec<NetworkProtocol> = observed
            .iter()
            .copied()
            .filter(|protocol| {
                self.blocked.contains(protocol)
                    || (!self.allowed.is_empty()
                        && !self.allowed.contains(protocol))
            })
            .collect();

        let status = if !unexpected.is_empty() {
            ProtocolLeakStatus::LeakDetected
        } else if observed.is_empty() && self.fail_closed {
            ProtocolLeakStatus::FailedClosed
        } else {
            ProtocolLeakStatus::Protected
        };

        ProtocolLeakTestResult {
            status,
            observed: observed.to_vec(),
            unexpected,
            duration: start.elapsed(),
        }
    }
}

impl Default for ProtocolLeakTest {
    fn default() -> Self {
        Self::new()
    }
}
