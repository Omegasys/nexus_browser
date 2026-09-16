// SPDX-License-Identifier: GPL-3.0-or-later

use super::doh::DohResolver;
use super::dot::DotResolver;
use super::dnscrypt::DnsCryptResolver;
use super::resolver::ResolverProtocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolverManagerState {
    Stopped,
    Running,
}

pub struct ResolverManager {
    state: ResolverManagerState,
    active_protocol: ResolverProtocol,
    doh: Option<DohResolver>,
    dot: Option<DotResolver>,
    dnscrypt: Option<DnsCryptResolver>,
}

impl ResolverManager {
    pub fn new() -> Self {
        Self {
            state: ResolverManagerState::Stopped,
            active_protocol: ResolverProtocol::Doh,
            doh: None,
            dot: None,
            dnscrypt: None,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.state = ResolverManagerState::Running;
        Ok(())
    }

    pub fn stop(&mut self) {
        self.state = ResolverManagerState::Stopped;
    }

    pub fn set_protocol(&mut self, protocol: ResolverProtocol) {
        self.active_protocol = protocol;
    }

    pub fn configure_doh(&mut self, resolver: DohResolver) {
        self.doh = Some(resolver);
    }

    pub fn configure_dot(&mut self, resolver: DotResolver) {
        self.dot = Some(resolver);
    }

    pub fn configure_dnscrypt(&mut self, resolver: DnsCryptResolver) {
        self.dnscrypt = Some(resolver);
    }

    pub fn resolve(&self, hostname: &str) -> Result<Vec<String>, String> {
        if self.state != ResolverManagerState::Running {
            return Err("Resolver manager is not running".into());
        }

        match self.active_protocol {
            ResolverProtocol::Doh => {
                self.doh
                    .as_ref()
                    .ok_or_else(|| "DoH resolver is not configured".into())?
                    .resolve(hostname)
            }

            ResolverProtocol::Dot => {
                self.dot
                    .as_ref()
                    .ok_or_else(|| "DoT resolver is not configured".into())?
                    .resolve(hostname)
            }

            ResolverProtocol::DnsCrypt => {
                self.dnscrypt
                    .as_ref()
                    .ok_or_else(|| "DNSCrypt resolver is not configured".into())?
                    .resolve(hostname)
            }

            ResolverProtocol::System
            | ResolverProtocol::PlainDns
            | ResolverProtocol::TorDns => {
                if hostname.is_empty() {
                    return Err("Hostname cannot be empty".into());
                }

                Ok(Vec::new())
            }
        }
    }

    pub fn state(&self) -> ResolverManagerState {
        self.state
    }

    pub fn active_protocol(&self) -> ResolverProtocol {
        self.active_protocol
    }
}

impl Default for ResolverManager {
    fn default() -> Self {
        Self::new()
    }
}
