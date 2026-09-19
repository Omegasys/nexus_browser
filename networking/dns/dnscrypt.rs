use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct DnsCryptResolver {
    enabled: bool,
    provider_name: String,
    provider_public_key: Option<String>,
    server_address: Option<IpAddr>,
    server_port: u16,
}

impl Default for DnsCryptResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl DnsCryptResolver {
    pub fn new() -> Self {
        Self {
            enabled: false,
            provider_name: "unconfigured".into(),
            provider_public_key: None,
            server_address: None,
            server_port: 443,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_provider_name(&mut self, name: impl Into<String>) {
        self.provider_name = name.into();
    }

    pub fn provider_name(&self) -> &str {
        &self.provider_name
    }

    pub fn set_provider_public_key(&mut self, key: impl Into<String>) {
        self.provider_public_key = Some(key.into());
    }

    pub fn clear_provider_public_key(&mut self) {
        self.provider_public_key = None;
    }

    pub fn provider_public_key(&self) -> Option<&str> {
        self.provider_public_key.as_deref()
    }

    pub fn set_server(&mut self, address: IpAddr, port: u16) {
        self.server_address = Some(address);
        self.server_port = port;
    }

    pub fn server(&self) -> Option<(IpAddr, u16)> {
        self.server_address.map(|address| {
            (address, self.server_port)
        })
    }

    pub fn resolve(&self, hostname: &str) -> Result<Vec<IpAddr>, String> {
        if !self.enabled {
            return Err("DNSCrypt resolver is disabled".into());
        }

        if hostname.is_empty() {
            return Err("Hostname cannot be empty".into());
        }

        if self.provider_public_key.is_none() {
            return Err("DNSCrypt provider key is not configured".into());
        }

        if self.server_address.is_none() {
            return Err("DNSCrypt server is not configured".into());
        }

        Err("DNSCrypt network transport is not implemented".into())
    }
}
