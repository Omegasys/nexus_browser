use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsLeakStatus {
    Protected,
    LeakDetected,
    Unavailable,
    FailedClosed,
}

#[derive(Debug, Clone)]
pub struct DnsLeakTestResult {
    pub status: DnsLeakStatus,
    pub resolver_addresses: Vec<IpAddr>,
    pub unexpected_resolvers: Vec<IpAddr>,
    pub duration: Duration,
    pub checked: bool,
}

impl DnsLeakTestResult {
    pub fn protected() -> Self {
        Self {
            status: DnsLeakStatus::Protected,
            resolver_addresses: Vec::new(),
            unexpected_resolvers: Vec::new(),
            duration: Duration::ZERO,
            checked: true,
        }
    }

    pub fn leak_detected(addresses: Vec<IpAddr>) -> Self {
        Self {
            status: DnsLeakStatus::LeakDetected,
            resolver_addresses: addresses.clone(),
            unexpected_resolvers: addresses,
            duration: Duration::ZERO,
            checked: true,
        }
    }

    pub fn failed_closed() -> Self {
        Self {
            status: DnsLeakStatus::FailedClosed,
            resolver_addresses: Vec::new(),
            unexpected_resolvers: Vec::new(),
            duration: Duration::ZERO,
            checked: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DnsLeakTest {
    allowed_resolvers: Vec<IpAddr>,
    fail_closed: bool,
    timeout: Duration,
}

impl DnsLeakTest {
    pub fn new() -> Self {
        Self {
            allowed_resolvers: Vec::new(),
            fail_closed: true,
            timeout: Duration::from_secs(5),
        }
    }

    pub fn allow_resolver(mut self, address: IpAddr) -> Self {
        self.allowed_resolvers.push(address);
        self
    }

    pub fn fail_closed(mut self, enabled: bool) -> Self {
        self.fail_closed = enabled;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn evaluate(
        &self,
        resolver_addresses: &[IpAddr],
    ) -> DnsLeakTestResult {
        let start = Instant::now();

        let unexpected: Vec<IpAddr> = resolver_addresses
            .iter()
            .copied()
            .filter(|address| !self.allowed_resolvers.contains(address))
            .collect();

        let status = if unexpected.is_empty() {
            if resolver_addresses.is_empty() && self.fail_closed {
                DnsLeakStatus::FailedClosed
            } else {
                DnsLeakStatus::Protected
            }
        } else {
            DnsLeakStatus::LeakDetected
        };

        DnsLeakTestResult {
            status,
            resolver_addresses: resolver_addresses.to_vec(),
            unexpected_resolvers: unexpected,
            duration: start.elapsed().min(self.timeout),
            checked: true,
        }
    }

    pub fn test_resolution(
        &self,
        hostname: &str,
    ) -> DnsLeakTestResult {
        let start = Instant::now();

        let socket = (hostname, 0)
            .to_socket_addrs();

        match socket {
            Ok(addresses) => {
                let resolved: Vec<IpAddr> =
                    addresses.map(|address| address.ip()).collect();

                let mut result = self.evaluate(&resolved);
                result.duration = start.elapsed().min(self.timeout);
                result
            }
            Err(_) => {
                if self.fail_closed {
                    DnsLeakTestResult {
                        status: DnsLeakStatus::FailedClosed,
                        resolver_addresses: Vec::new(),
                        unexpected_resolvers: Vec::new(),
                        duration: start.elapsed().min(self.timeout),
                        checked: true,
                    }
                } else {
                    DnsLeakTestResult {
                        status: DnsLeakStatus::Unavailable,
                        resolver_addresses: Vec::new(),
                        unexpected_resolvers: Vec::new(),
                        duration: start.elapsed().min(self.timeout),
                        checked: false,
                    }
                }
            }
        }
    }
}

impl Default for DnsLeakTest {
    fn default() -> Self {
        Self::new()
    }
}
