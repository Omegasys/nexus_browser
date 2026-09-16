// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolverProtocol {
    System,
    PlainDns,
    Doh,
    Dot,
    DnsCrypt,
    TorDns,
}

pub trait DnsResolver {
    fn protocol(&self) -> ResolverProtocol;

    fn is_enabled(&self) -> bool;

    fn resolve(&self, hostname: &str) -> Result<Vec<String>, String>;
}

#[derive(Debug, Clone)]
pub struct ResolverConfig {
    pub protocol: ResolverProtocol,
    pub endpoint: String,
    pub enabled: bool,
}

impl ResolverConfig {
    pub fn new(
        protocol: ResolverProtocol,
        endpoint: impl Into<String>,
    ) -> Self {
        Self {
            protocol,
            endpoint: endpoint.into(),
            enabled: true,
        }
    }
}
