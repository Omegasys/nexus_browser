#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsKillSwitchState {
    Disabled,
    Armed,
    Triggered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DnsKillSwitchLayer {
    AllDns,
    SystemDns,
    DoH,
    DoT,
    DnsCrypt,
    TorDns,
    PlaintextDns,
    ResolverBypass,
}

#[derive(Debug)]
pub struct DnsKillSwitch {
    state: DnsKillSwitchState,
    blocked_layers: Vec<DnsKillSwitchLayer>,
    fail_closed: bool,
}

impl Default for DnsKillSwitch {
    fn default() -> Self {
        Self::new()
    }
}

impl DnsKillSwitch {
    pub fn new() -> Self {
        Self {
            state: DnsKillSwitchState::Disabled,
            blocked_layers: Vec::new(),
            fail_closed: true,
        }
    }

    pub fn arm(&mut self) {
        self.state = DnsKillSwitchState::Armed;
    }

    pub fn trigger(&mut self) {
        self.state = DnsKillSwitchState::Triggered;
    }

    pub fn disable(&mut self) {
        self.state = DnsKillSwitchState::Disabled;
    }

    pub fn state(&self) -> DnsKillSwitchState {
        self.state
    }

    pub fn set_fail_closed(&mut self, enabled: bool) {
        self.fail_closed = enabled;
    }

    pub fn fail_closed(&self) -> bool {
        self.fail_closed
    }

    pub fn block_layer(&mut self, layer: DnsKillSwitchLayer) {
        if !self.blocked_layers.contains(&layer) {
            self.blocked_layers.push(layer);
        }
    }

    pub fn unblock_layer(&mut self, layer: DnsKillSwitchLayer) {
        self.blocked_layers.retain(|item| *item != layer);
    }

    pub fn clear_layers(&mut self) {
        self.blocked_layers.clear();
    }

    pub fn is_blocked(&self, layer: DnsKillSwitchLayer) -> bool {
        if self.state == DnsKillSwitchState::Triggered {
            return true;
        }

        if self.blocked_layers.contains(&DnsKillSwitchLayer::AllDns) {
            return true;
        }

        self.blocked_layers.contains(&layer)
    }

    pub fn permits(&self, layer: DnsKillSwitchLayer) -> bool {
        if self.state == DnsKillSwitchState::Triggered {
            return false;
        }

        if self.state == DnsKillSwitchState::Disabled {
            return !self.fail_closed;
        }

        !self.is_blocked(layer)
    }

    pub fn blocked_layers(&self) -> &[DnsKillSwitchLayer] {
        &self.blocked_layers
    }
}
