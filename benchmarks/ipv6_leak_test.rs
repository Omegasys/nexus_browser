use std::net::Ipv6Addr;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ipv6LeakStatus {
    Protected,
    LeakDetected,
    Disabled,
    FailedClosed,
}

#[derive(Debug, Clone)]
pub struct Ipv6LeakTestResult {
    pub status: Ipv6LeakStatus,
    pub observed_addresses: Vec<Ipv6Addr>,
    pub unexpected_addresses: Vec<Ipv6Addr>,
    pub duration: Duration,
}

impl Ipv6LeakTestResult {
    pub fn protected() -> Self {
        Self {
            status: Ipv6LeakStatus::Protected,
            observed_addresses: Vec::new(),
            unexpected_addresses: Vec::new(),
            duration: Duration::ZERO,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ipv6LeakTest {
    allowed_addresses: Vec<Ipv6Addr>,
    ipv6_enabled: bool,
    fail_closed: bool,
}

impl Ipv6LeakTest {
    pub fn new() -> Self {
        Self {
            allowed_addresses: Vec::new(),
            ipv6_enabled: true,
            fail_closed: true,
        }
    }

    pub fn allow_address(mut self, address: Ipv6Addr) -> Self {
        self.allowed_addresses.push(address);
        self
    }

    pub fn ipv6_enabled(mut self, enabled: bool) -> Self {
        self.ipv6_enabled = enabled;
        self
    }

    pub fn fail_closed(mut self, enabled: bool) -> Self {
        self.fail_closed = enabled;
        self
    }

    pub fn evaluate(
        &self,
        observed_addresses: &[Ipv6Addr],
    ) -> Ipv6LeakTestResult {
        let start = Instant::now();

        if !self.ipv6_enabled {
            return Ipv6LeakTestResult {
                status: if observed_addresses.is_empty() {
                    Ipv6LeakStatus::Disabled
                } else {
                    Ipv6LeakStatus::LeakDetected
                },
                observed_addresses: observed_addresses.to_vec(),
                unexpected_addresses: observed_addresses.to_vec(),
                duration: start.elapsed(),
            };
        }

        let unexpected: Vec<Ipv6Addr> = observed_addresses
            .iter()
            .copied()
            .filter(|address| !self.allowed_addresses.contains(address))
            .collect();

        let status = if unexpected.is_empty() {
            if observed_addresses.is_empty() && self.fail_closed {
                Ipv6LeakStatus::FailedClosed
            } else {
                Ipv6LeakStatus::Protected
            }
        } else {
            Ipv6LeakStatus::LeakDetected
        };

        Ipv6LeakTestResult {
            status,
            observed_addresses: observed_addresses.to_vec(),
            unexpected_addresses: unexpected,
            duration: start.elapsed(),
        }
    }
}

impl Default for Ipv6LeakTest {
    fn default() -> Self {
        Self::new()
    }
}
