// SPDX-License-Identifier: GPL-3.0-or-later

use super::dns_cache::DnsCache;
use super::dns_kill_switch::DnsKillSwitch;
use super::dns_leak_protection::DnsLeakProtection;
use super::dns_partitioning::DnsPartitionManager;
use super::resolver::DnsResolver;
use super::resolver_manager::ResolverManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsEngineState {
    Disabled,
    Starting,
    Running,
    Locked,
    Error,
}

pub struct DnsEngine {
    state: DnsEngineState,
    resolver_manager: ResolverManager,
    cache: DnsCache,
    partitioning: DnsPartitionManager,
    leak_protection: DnsLeakProtection,
    kill_switch: DnsKillSwitch,
}

impl DnsEngine {
    pub fn new() -> Self {
        Self {
            state: DnsEngineState::Disabled,
            resolver_manager: ResolverManager::new(),
            cache: DnsCache::new(),
            partitioning: DnsPartitionManager::new(),
            leak_protection: DnsLeakProtection::new(),
            kill_switch: DnsKillSwitch::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.kill_switch.is_locked() {
            return Err("DNS kill switch is locked".into());
        }

        if !self.leak_protection.is_ready() {
            return Err("DNS leak protection is not ready".into());
        }

        self.state = DnsEngineState::Starting;
        self.resolver_manager.start()?;
        self.state = DnsEngineState::Running;

        Ok(())
    }

    pub fn stop(&mut self) {
        self.resolver_manager.stop();
        self.state = DnsEngineState::Disabled;
    }

    pub fn lock(&mut self) {
        self.kill_switch.enable();
        self.state = DnsEngineState::Locked;
    }

    pub fn unlock(&mut self) {
        self.kill_switch.disable();

        if self.state == DnsEngineState::Locked {
            self.state = DnsEngineState::Disabled;
        }
    }

    pub fn resolve(&mut self, hostname: &str) -> Result<Vec<String>, String> {
        if self.state != DnsEngineState::Running {
            return Err("DNS engine is not running".into());
        }

        if self.kill_switch.is_locked() {
            return Err("DNS resolution blocked by kill switch".into());
        }

        if let Some(cached) = self.cache.get(hostname) {
            return Ok(cached);
        }

        let addresses = self.resolver_manager.resolve(hostname)?;

        self.cache.insert(hostname, addresses.clone());

        Ok(addresses)
    }

    pub fn state(&self) -> DnsEngineState {
        self.state
    }

    pub fn resolver_manager(&self) -> &ResolverManager {
        &self.resolver_manager
    }

    pub fn resolver_manager_mut(&mut self) -> &mut ResolverManager {
        &mut self.resolver_manager
    }

    pub fn cache(&self) -> &DnsCache {
        &self.cache
    }

    pub fn cache_mut(&mut self) -> &mut DnsCache {
        &mut self.cache
    }

    pub fn partitioning(&self) -> &DnsPartitionManager {
        &self.partitioning
    }

    pub fn partitioning_mut(&mut self) -> &mut DnsPartitionManager {
        &mut self.partitioning
    }

    pub fn leak_protection(&self) -> &DnsLeakProtection {
        &self.leak_protection
    }

    pub fn kill_switch(&self) -> &DnsKillSwitch {
        &self.kill_switch
    }
}

impl Default for DnsEngine {
    fn default() -> Self {
        Self::new()
    }
}
