use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KillSwitchLayer {
    Network,
    Direct,
    Proxy,
    Vpn,
    Tor,
    I2p,
    Nym,
    Lokinet,
    Yggdrasil,
    Dns,
    Ipv4,
    Ipv6,
    Tcp,
    Udp,
    Quic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KillSwitchState {
    Disabled,
    Enabled,
    Triggered,
}

#[derive(Debug, Clone)]
pub struct KillSwitch {
    pub state: KillSwitchState,
    blocked_layers: HashSet<KillSwitchLayer>,
}

impl KillSwitch {
    pub fn new() -> Self {
        Self {
            state: KillSwitchState::Disabled,
            blocked_layers: HashSet::new(),
        }
    }

    pub fn enable(&mut self) {
        self.state = KillSwitchState::Enabled;
    }

    pub fn disable(&mut self) {
        self.state = KillSwitchState::Disabled;
        self.blocked_layers.clear();
    }

    pub fn trigger(&mut self) {
        self.state = KillSwitchState::Triggered;
    }

    pub fn block_layer(&mut self, layer: KillSwitchLayer) {
        self.blocked_layers.insert(layer);

        if self.state == KillSwitchState::Disabled {
            self.state = KillSwitchState::Enabled;
        }
    }

    pub fn unblock_layer(&mut self, layer: KillSwitchLayer) {
        self.blocked_layers.remove(&layer);
    }

    pub fn is_blocked(&self, layer: KillSwitchLayer) -> bool {
        self.state == KillSwitchState::Triggered
            || self.blocked_layers.contains(&layer)
    }

    pub fn blocks_all_network(&self) -> bool {
        self.state == KillSwitchState::Triggered
            || self.blocked_layers.contains(&KillSwitchLayer::Network)
    }

    pub fn blocked_layers(&self) -> impl Iterator<Item = &KillSwitchLayer> {
        self.blocked_layers.iter()
    }
}

impl Default for KillSwitch {
    fn default() -> Self {
        Self::new()
    }
}
