// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TorrentSecurityLevel {
    Standard,
    Strict,
    Locked,
}

pub struct TorrentSecurity {
    level: TorrentSecurityLevel,
    block_private_addresses: bool,
    require_network_isolation: bool,
    allow_inbound_connections: bool,
}

impl TorrentSecurity {
    pub fn new() -> Self {
        Self {
            level: TorrentSecurityLevel::Strict,
            block_private_addresses: false,
            require_network_isolation: true,
            allow_inbound_connections: true,
        }
    }

    pub fn set_level(&mut self, level: TorrentSecurityLevel) {
        self.level = level;

        if level == TorrentSecurityLevel::Locked {
            self.allow_inbound_connections = false;
            self.require_network_isolation = true;
        }
    }

    pub fn set_block_private_addresses(&mut self, blocked: bool) {
        self.block_private_addresses = blocked;
    }

    pub fn set_allow_inbound(&mut self, allowed: bool) {
        self.allow_inbound_connections = allowed;
    }

    pub fn is_safe(&self) -> bool {
        match self.level {
            TorrentSecurityLevel::Standard => true,
            TorrentSecurityLevel::Strict => self.require_network_isolation,
            TorrentSecurityLevel::Locked => {
                self.require_network_isolation
                    && !self.allow_inbound_connections
            }
        }
    }

    pub fn level(&self) -> TorrentSecurityLevel {
        self.level
    }

    pub fn blocks_private_addresses(&self) -> bool {
        self.block_private_addresses
    }

    pub fn requires_network_isolation(&self) -> bool {
        self.require_network_isolation
    }

    pub fn allows_inbound_connections(&self) -> bool {
        self.allow_inbound_connections
    }
}

impl Default for TorrentSecurity {
    fn default() -> Self {
        Self::new()
    }
}
