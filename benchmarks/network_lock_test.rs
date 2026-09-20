use std::collections::HashSet;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NetworkLockLayer {
    AllTraffic,
    Dns,
    Ipv4,
    Ipv6,
    Tcp,
    Udp,
    Quic,
    DirectConnection,
    ProxyBypass,
    VpnBypass,
    TorBypass,
    WebRtc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkLockStatus {
    Locked,
    LeakDetected,
    PartiallyLocked,
    Unlocked,
}

#[derive(Debug, Clone)]
pub struct NetworkLockTestResult {
    pub status: NetworkLockStatus,
    pub enabled_layers: Vec<NetworkLockLayer>,
    pub violated_layers: Vec<NetworkLockLayer>,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct NetworkLockTest {
    enabled_layers: HashSet<NetworkLockLayer>,
    fail_closed: bool,
}

impl NetworkLockTest {
    pub fn new() -> Self {
        Self {
            enabled_layers: HashSet::new(),
            fail_closed: true,
        }
    }

    pub fn enable(mut self, layer: NetworkLockLayer) -> Self {
        self.enabled_layers.insert(layer);
        self
    }

    pub fn disable(&mut self, layer: NetworkLockLayer) {
        self.enabled_layers.remove(&layer);
    }

    pub fn fail_closed(mut self, enabled: bool) -> Self {
        self.fail_closed = enabled;
        self
    }

    pub fn evaluate(
        &self,
        violated_layers: &[NetworkLockLayer],
    ) -> NetworkLockTestResult {
        let start = Instant::now();

        let violations: Vec<NetworkLockLayer> = violated_layers
            .iter()
            .copied()
            .filter(|layer| self.enabled_layers.contains(layer))
            .collect();

        let status = if self.enabled_layers.is_empty() {
            NetworkLockStatus::Unlocked
        } else if !violations.is_empty() {
            NetworkLockStatus::LeakDetected
        } else if self.fail_closed {
            NetworkLockStatus::Locked
        } else {
            NetworkLockStatus::PartiallyLocked
        };

        NetworkLockTestResult {
            status,
            enabled_layers: self.enabled_layers.iter().copied().collect(),
            violated_layers: violations,
            duration: start.elapsed(),
        }
    }

    pub fn is_layer_locked(&self, layer: NetworkLockLayer) -> bool {
        self.enabled_layers.contains(&layer)
    }

    pub fn locked_layers(&self) -> impl Iterator<Item = &NetworkLockLayer> {
        self.enabled_layers.iter()
    }

    pub fn clear(&mut self) {
        self.enabled_layers.clear();
    }
}

impl Default for NetworkLockTest {
    fn default() -> Self {
        Self::new()
    }
}
