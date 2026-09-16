// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TorrentNetwork {
    Direct,
    Proxy,
    Vpn,
    I2p,
    Nym,
    Lokinet,
    Blocked,
}

pub struct NetworkIsolation {
    enabled: bool,
    selected_network: TorrentNetwork,
    allow_fallback: bool,
    leak_protection: bool,
}

impl NetworkIsolation {
    pub fn new() -> Self {
        Self {
            enabled: true,
            selected_network: TorrentNetwork::Vpn,
            allow_fallback: false,
            leak_protection: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn set_network(&mut self, network: TorrentNetwork) {
        self.selected_network = network;
    }

    pub fn set_fallback(&mut self, enabled: bool) {
        self.allow_fallback = enabled;
    }

    pub fn set_leak_protection(&mut self, enabled: bool) {
        self.leak_protection = enabled;
    }

    pub fn is_ready(&self) -> bool {
        self.enabled
            && self.leak_protection
            && self.selected_network != TorrentNetwork::Blocked
    }

    pub fn should_allow_connection(&self) -> bool {
        self.is_ready()
    }

    pub fn network(&self) -> TorrentNetwork {
        self.selected_network
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn fallback_enabled(&self) -> bool {
        self.allow_fallback
    }

    pub fn leak_protection_enabled(&self) -> bool {
        self.leak_protection
    }
}

impl Default for NetworkIsolation {
    fn default() -> Self {
        Self::new()
    }
}
