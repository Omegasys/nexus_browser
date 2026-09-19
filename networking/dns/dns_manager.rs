use std::net::IpAddr;

use super::dnscrypt::DnsCryptResolver;
use super::dnssec::{DnssecMode, DnssecValidator};
use super::doh::DohResolver;
use super::dot::DotResolver;
use super::secure_dns::{DnsMode, SecureDnsConfig};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsManagerState {
    Disabled,
    Starting,
    Running,
    Locked,
    Error,
}

#[derive(Debug)]
pub struct DnsManager {
    state: DnsManagerState,
    config: SecureDnsConfig,
    doh: DohResolver,
    dot: DotResolver,
    dnscrypt: DnsCryptResolver,
    dnssec: DnssecValidator,
}

impl Default for DnsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DnsManager {
    pub fn new() -> Self {
        Self {
            state: DnsManagerState::Disabled,
            config: SecureDnsConfig::default(),
            doh: DohResolver::new(),
            dot: DotResolver::new(),
            dnscrypt: DnsCryptResolver::new(),
            dnssec: DnssecValidator::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.state == DnsManagerState::Locked {
            return Err("DNS manager is locked".into());
        }

        self.state = DnsManagerState::Starting;

        match self.config.mode {
            DnsMode::Disabled => {
                self.state = DnsManagerState::Disabled;
            }
            DnsMode::System => {
                if self.config.allow_plaintext_fallback {
                    self.state = DnsManagerState::Running;
                } else {
                    self.state = DnsManagerState::Error;
                    return Err(
                        "System DNS is disabled because plaintext fallback is not allowed"
                            .into(),
                    );
                }
            }
            DnsMode::DoH => {
                self.doh.set_enabled(true);
                self.state = DnsManagerState::Running;
            }
            DnsMode::DoT => {
                self.dot.set_enabled(true);
                self.state = DnsManagerState::Running;
            }
            DnsMode::DnsCrypt => {
                self.dnscrypt.set_enabled(true);
                self.state = DnsManagerState::Running;
            }
            DnsMode::Tor => {
                self.state = DnsManagerState::Running;
            }
        }

        Ok(())
    }

    pub fn stop(&mut self) {
        self.doh.set_enabled(false);
        self.dot.set_enabled(false);
        self.dnscrypt.set_enabled(false);

        self.state = DnsManagerState::Disabled;
    }

    pub fn lock(&mut self) {
        self.stop();
        self.state = DnsManagerState::Locked;
    }

    pub fn unlock(&mut self) {
        if self.state == DnsManagerState::Locked {
            self.state = DnsManagerState::Disabled;
        }
    }

    pub fn state(&self) -> DnsManagerState {
        self.state
    }

    pub fn configure(&mut self, config: SecureDnsConfig) {
        self.config = config;
    }

    pub fn config(&self) -> &SecureDnsConfig {
        &self.config
    }

    pub fn resolve(&self, hostname: &str) -> Result<Vec<IpAddr>, String> {
        if self.state != DnsManagerState::Running {
            return Err("DNS manager is not running".into());
        }

        if hostname.is_empty() {
            return Err("Hostname cannot be empty".into());
        }

        match self.config.mode {
            DnsMode::DoH => self.doh.resolve(hostname),
            DnsMode::DoT => self.dot.resolve(hostname),
            DnsMode::DnsCrypt => self.dnscrypt.resolve(hostname),
            DnsMode::System => {
                if self.config.allow_plaintext_fallback {
                    Err("System resolver integration is not implemented".into())
                } else {
                    Err("Plaintext DNS is disabled".into())
                }
            }
            DnsMode::Tor => {
                Err("Tor DNS integration is not implemented".into())
            }
            DnsMode::Disabled => Err("DNS is disabled".into()),
        }
    }

    pub fn dnssec(&self) -> &DnssecValidator {
        &self.dnssec
    }

    pub fn dnssec_mut(&mut self) -> &mut DnssecValidator {
        &mut self.dnssec
    }

    pub fn set_dnssec_mode(&mut self, mode: DnssecMode) {
        self.dnssec.set_mode(mode);
    }
}
