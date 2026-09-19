use std::collections::HashSet;
use std::net::IpAddr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LeakType {
    Dns,
    Ipv4,
    Ipv6,
    WebRtc,
    DirectConnection,
    ProxyBypass,
    VpnBypass,
    TorBypass,
    InterfaceExposure,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeakSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct LeakEvent {
    pub leak_type: LeakType,
    pub severity: LeakSeverity,
    pub address: Option<IpAddr>,
    pub description: String,
    pub blocked: bool,
}

impl LeakEvent {
    pub fn new(
        leak_type: LeakType,
        severity: LeakSeverity,
        description: impl Into<String>,
    ) -> Self {
        Self {
            leak_type,
            severity,
            address: None,
            description: description.into(),
            blocked: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct LeakDetector {
    detected: Vec<LeakEvent>,
    protected_ips: HashSet<IpAddr>,
    fail_closed: bool,
}

impl LeakDetector {
    pub fn new() -> Self {
        Self {
            detected: Vec::new(),
            protected_ips: HashSet::new(),
            fail_closed: true,
        }
    }

    pub fn set_fail_closed(&mut self, enabled: bool) {
        self.fail_closed = enabled;
    }

    pub fn fail_closed(&self) -> bool {
        self.fail_closed
    }

    pub fn add_protected_ip(&mut self, address: IpAddr) {
        self.protected_ips.insert(address);
    }

    pub fn is_protected_ip(&self, address: &IpAddr) -> bool {
        self.protected_ips.contains(address)
    }

    pub fn report(&mut self, event: LeakEvent) {
        self.detected.push(event);
    }

    pub fn report_ip_leak(
        &mut self,
        leak_type: LeakType,
        address: IpAddr,
        severity: LeakSeverity,
    ) {
        let mut event = LeakEvent::new(
            leak_type,
            severity,
            format!("Unexpected network address exposure: {address}"),
        );

        event.address = Some(address);
        event.blocked = self.fail_closed;

        self.report(event);
    }

    pub fn has_leaks(&self) -> bool {
        !self.detected.is_empty()
    }

    pub fn critical_leak_detected(&self) -> bool {
        self.detected.iter().any(|event| {
            event.severity == LeakSeverity::Critical
        })
    }

    pub fn detected(&self) -> &[LeakEvent] {
        &self.detected
    }

    pub fn clear(&mut self) {
        self.detected.clear();
    }
}
