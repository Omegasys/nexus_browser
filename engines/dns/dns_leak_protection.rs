// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsLeakState {
    Protected,
    Unprotected,
    Locked,
}

pub struct DnsLeakProtection {
    state: DnsLeakState,
    block_plain_dns: bool,
    block_system_resolver: bool,
    enforce_selected_resolver: bool,
}

impl DnsLeakProtection {
    pub fn new() -> Self {
        Self {
            state: DnsLeakState::Protected,
            block_plain_dns: true,
            block_system_resolver: true,
            enforce_selected_resolver: true,
        }
    }

    pub fn enable(&mut self) {
        self.state = DnsLeakState::Protected;
    }

    pub fn disable(&mut self) {
        self.state = DnsLeakState::Unprotected;
    }

    pub fn lock(&mut self) {
        self.state = DnsLeakState::Locked;
    }

    pub fn set_block_plain_dns(&mut self, blocked: bool) {
        self.block_plain_dns = blocked;
    }

    pub fn set_block_system_resolver(&mut self, blocked: bool) {
        self.block_system_resolver = blocked;
    }

    pub fn set_enforce_selected_resolver(&mut self, enforce: bool) {
        self.enforce_selected_resolver = enforce;
    }

    pub fn is_ready(&self) -> bool {
        self.state == DnsLeakState::Protected
    }

    pub fn allows_plain_dns(&self) -> bool {
        !self.block_plain_dns
    }

    pub fn allows_system_resolver(&self) -> bool {
        !self.block_system_resolver
    }

    pub fn enforces_selected_resolver(&self) -> bool {
        self.enforce_selected_resolver
    }

    pub fn state(&self) -> DnsLeakState {
        self.state
    }
}

impl Default for DnsLeakProtection {
    fn default() -> Self {
        Self::new()
    }
}
