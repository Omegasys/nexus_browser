use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ipv4LeakStatus {
    Protected,
    LeakDetected,
    Disabled,
    FailedClosed,
}

#[derive(Debug, Clone)]
pub struct Ipv4LeakTestResult {
    pub status: Ipv4LeakStatus,
    pub observed_addresses: Vec<Ipv4Addr>,
    pub unexpected_addresses: Vec<Ipv4Addr>,
    pub duration: Duration,
}

impl Ipv4LeakTestResult {
    pub fn protected() -> Self {
        Self {
            status: Ipv4LeakStatus::Protected,
            observed_addresses: Vec::new(),
            unexpected_addresses: Vec::new(),
            duration: Duration::ZERO,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ipv4LeakTest {
    allowed_addresses: Vec<Ipv4Addr>,
    ipv4_enabled: bool,
    fail_closed: bool,
}

impl Ipv4LeakTest {
    pub fn new() -> Self {
        Self {
            allowed_addresses: Vec::new(),
            ipv4_enabled: true,
            fail_closed: true,
        }
    }

    pub fn allow_address(mut self, address: Ipv4Addr) -> Self {
        self.allowed_addresses.push(address);
        self
    }

    pub fn ipv4_enabled(mut self, enabled: bool) -> Self {
        self.ipv4_enabled = enabled;
        self
    }

    pub fn fail_closed(mut self, enabled: bool) -> Self {
        self.fail_closed = enabled;
        self
    }

    pub fn evaluate(
        &self,
        observed_addresses: &[Ipv4Addr],
    ) -> Ipv4LeakTestResult {
        let start = Instant::now();

        if !self.ipv4_enabled {
            return Ipv4LeakTestResult {
                status: if observed_addresses.is_empty() {
                    Ipv4LeakStatus::Disabled
                } else {
                    Ipv4LeakStatus::LeakDetected
                },
                observed_addresses: observed_addresses.to_vec(),
                unexpected_addresses: observed_addresses.to_vec(),
                duration: start.elapsed(),
            };
        }

        let unexpected: Vec<Ipv4Addr> = observed_addresses
            .iter()
            .copied()
            .filter(|address| !self.allowed_addresses.contains(address))
            .collect();

        let status = if unexpected.is_empty() {
            if observed_addresses.is_empty() && self.fail_closed {
                Ipv4LeakStatus::FailedClosed
            } else {
                Ipv4LeakStatus::Protected
            }
        } else {
            Ipv4LeakStatus::LeakDetected
        };

        Ipv4LeakTestResult {
            status,
            observed_addresses: observed_addresses.to_vec(),
            unexpected_addresses: unexpected,
            duration: start.elapsed(),
        }
    }
}

impl Default for Ipv4LeakTest {
    fn default() -> Self {
        Self::new()
    }
}
