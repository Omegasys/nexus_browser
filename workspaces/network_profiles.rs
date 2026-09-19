use std::collections::HashMap;

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

#[derive(Debug, Clone)]
pub struct NetworkProfile {
    pub name: String,
    pub route: NetworkRoute,

    pub ipv4_enabled: bool,
    pub ipv6_enabled: bool,
    pub tcp_enabled: bool,
    pub udp_enabled: bool,
    pub quic_enabled: bool,

    pub dns_secure: bool,
    pub dns_fail_closed: bool,
    pub kill_switch: bool,
    pub leak_prevention: bool,

    pub allow_fallback: bool,
    pub custom_endpoint: Option<String>,
}

impl NetworkProfile {
    pub fn direct() -> Self {
        Self {
            name: "Direct".to_string(),
            route: NetworkRoute::Direct,

            ipv4_enabled: true,
            ipv6_enabled: true,
            tcp_enabled: true,
            udp_enabled: true,
            quic_enabled: true,

            dns_secure: true,
            dns_fail_closed: false,
            kill_switch: false,
            leak_prevention: true,

            allow_fallback: true,
            custom_endpoint: None,
        }
    }

    pub fn vpn() -> Self {
        Self {
            name: "VPN".to_string(),
            route: NetworkRoute::Vpn,

            ipv4_enabled: true,
            ipv6_enabled: true,
            tcp_enabled: true,
            udp_enabled: true,
            quic_enabled: true,

            dns_secure: true,
            dns_fail_closed: true,
            kill_switch: true,
            leak_prevention: true,

            allow_fallback: false,
            custom_endpoint: None,
        }
    }

    pub fn tor() -> Self {
        Self {
            name: "Tor".to_string(),
            route: NetworkRoute::Tor,

            ipv4_enabled: true,
            ipv6_enabled: false,
            tcp_enabled: true,
            udp_enabled: false,
            quic_enabled: false,

            dns_secure: true,
            dns_fail_closed: true,
            kill_switch: true,
            leak_prevention: true,

            allow_fallback: false,
            custom_endpoint: None,
        }
    }

    pub fn private() -> Self {
        Self {
            name: "Private".to_string(),
            route: NetworkRoute::Vpn,

            ipv4_enabled: true,
            ipv6_enabled: true,
            tcp_enabled: true,
            udp_enabled: true,
            quic_enabled: true,

            dns_secure: true,
            dns_fail_closed: true,
            kill_switch: true,
            leak_prevention: true,

            allow_fallback: false,
            custom_endpoint: None,
        }
    }

    pub fn set_endpoint(
        &mut self,
        endpoint: impl Into<String>,
    ) {
        self.custom_endpoint = Some(endpoint.into());
    }

    pub fn clear_endpoint(&mut self) {
        self.custom_endpoint = None;
    }
}

#[derive(Debug, Default)]
pub struct NetworkProfileManager {
    profiles: HashMap<String, NetworkProfile>,
}

impl NetworkProfileManager {
    pub fn new() -> Self {
        let mut manager = Self {
            profiles: HashMap::new(),
        };

        manager.register(NetworkProfile::direct());
        manager.register(NetworkProfile::vpn());
        manager.register(NetworkProfile::tor());
        manager.register(NetworkProfile::private());

        manager
    }

    pub fn register(&mut self, profile: NetworkProfile) {
        self.profiles
            .insert(profile.name.clone(), profile);
    }

    pub fn get(&self, name: &str) -> Option<&NetworkProfile> {
        self.profiles.get(name)
    }

    pub fn get_mut(
        &mut self,
        name: &str,
    ) -> Option<&mut NetworkProfile> {
        self.profiles.get_mut(name)
    }

    pub fn remove(
        &mut self,
        name: &str,
    ) -> Option<NetworkProfile> {
        self.profiles.remove(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.profiles.contains_key(name)
    }

    pub fn names(&self) -> Vec<String> {
        self.profiles.keys().cloned().collect()
    }

    pub fn count(&self) -> usize {
        self.profiles.len()
    }
}
