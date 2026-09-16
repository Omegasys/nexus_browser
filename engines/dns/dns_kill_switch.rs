// SPDX-License-Identifier: GPL-3.0-or-later

pub struct DnsKillSwitch {
    enabled: bool,
    block_on_failure: bool,
    block_plain_dns: bool,
}

impl DnsKillSwitch {
    pub fn new() -> Self {
        Self {
            enabled: false,
            block_on_failure: true,
            block_plain_dns: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn trigger(&mut self) {
        self.enabled = true;
    }

    pub fn reset(&mut self) {
        self.enabled = false;
    }

    pub fn is_locked(&self) -> bool {
        self.enabled
    }

    pub fn set_block_on_failure(&mut self, enabled: bool) {
        self.block_on_failure = enabled;
    }

    pub fn set_block_plain_dns(&mut self, enabled: bool) {
        self.block_plain_dns = enabled;
    }

    pub fn blocks_on_failure(&self) -> bool {
        self.block_on_failure
    }

    pub fn blocks_plain_dns(&self) -> bool {
        self.block_plain_dns
    }
}

impl Default for DnsKillSwitch {
    fn default() -> Self {
        Self::new()
    }
}
