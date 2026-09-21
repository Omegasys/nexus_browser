#[derive(Debug, Clone)]
pub struct NetworkSettings {
    pub route: NetworkRoute,
    pub dns_mode: DnsMode,
    pub proxy_mode: ProxyMode,
    pub proxy_address: Option<String>,
    pub vpn_interface: Option<String>,
    pub tor_socks_address: Option<String>,
    pub i2p_address: Option<String>,
    pub nym_address: Option<String>,
    pub lokinet_address: Option<String>,
    pub ipv4_enabled: bool,
    pub ipv6_enabled: bool,
    pub tcp_enabled: bool,
    pub udp_enabled: bool,
    pub quic_enabled: bool,
    pub webrtc_enabled: bool,
    pub network_lock: bool,
    pub fail_closed: bool,
    pub prevent_direct_fallback: bool,
    pub connection_timeout_seconds: u64,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            route: NetworkRoute::Direct,
            dns_mode: DnsMode::DoH,
            proxy_mode: ProxyMode::Disabled,
            proxy_address: None,
            vpn_interface: None,
            tor_socks_address: None,
            i2p_address: None,
            nym_address: None,
            lokinet_address: None,
            ipv4_enabled: true,
            ipv6_enabled: true,
            tcp_enabled: true,
            udp_enabled: true,
            quic_enabled: true,
            webrtc_enabled: true,
            network_lock: true,
            fail_closed: true,
            prevent_direct_fallback: true,
            connection_timeout_seconds: 30,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkRoute {
    Direct,
    Proxy,
    Vpn,
    Tor,
    I2p,
    Nym,
    Lokinet,
    Yggdrasil,
    Custom,
}

impl NetworkRoute {
    pub fn requires_external_backend(&self) -> bool {
        !matches!(self, Self::Direct)
    }

    pub fn supports_direct_fallback(&self) -> bool {
        matches!(self, Self::Direct)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsMode {
    Disabled,
    System,
    DoH,
    DoT,
    DnsCrypt,
    Tor,
}

impl DnsMode {
    pub fn encrypted(&self) -> bool {
        matches!(
            self,
            Self::DoH | Self::DoT | Self::DnsCrypt | Self::Tor
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyMode {
    Disabled,
    Http,
    Https,
    Socks4,
    Socks5,
    Pac,
}

impl ProxyMode {
    pub fn enabled(&self) -> bool {
        !matches!(self, Self::Disabled)
    }
}

impl NetworkSettings {
    pub fn route_requires_dns_isolation(&self) -> bool {
        matches!(
            self.route,
            NetworkRoute::Tor
                | NetworkRoute::I2p
                | NetworkRoute::Nym
                | NetworkRoute::Lokinet
        )
    }

    pub fn direct_connections_allowed(&self) -> bool {
        matches!(self.route, NetworkRoute::Direct) && !self.prevent_direct_fallback
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.connection_timeout_seconds == 0 {
            return Err("connection timeout must be greater than zero".to_string());
        }

        if self.route_requires_dns_isolation()
            && matches!(self.dns_mode, DnsMode::System)
        {
            return Err(
                "the selected network route requires isolated DNS".to_string()
            );
        }

        if self.fail_closed && !self.network_lock {
            return Err(
                "fail-closed operation requires the network lock".to_string()
            );
        }

        if self.proxy_mode.enabled() && self.proxy_address.is_none() {
            return Err(
                "a proxy address is required when a proxy is enabled".to_string()
            );
        }

        Ok(())
    }
}
