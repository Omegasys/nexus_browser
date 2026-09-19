use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct DohResolver {
    enabled: bool,
    endpoint: String,
    bootstrap_addresses: Vec<IpAddr>,
    use_post: bool,
}

impl Default for DohResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl DohResolver {
    pub fn new() -> Self {
        Self {
            enabled: false,
            endpoint: "https://dns.example.invalid/dns-query".into(),
            bootstrap_addresses: Vec::new(),
            use_post: true,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_endpoint(&mut self, endpoint: impl Into<String>) {
        self.endpoint = endpoint.into();
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn add_bootstrap_address(&mut self, address: IpAddr) {
        if !self.bootstrap_addresses.contains(&address) {
            self.bootstrap_addresses.push(address);
        }
    }

    pub fn clear_bootstrap_addresses(&mut self) {
        self.bootstrap_addresses.clear();
    }

    pub fn bootstrap_addresses(&self) -> &[IpAddr] {
        &self.bootstrap_addresses
    }

    pub fn set_use_post(&mut self, enabled: bool) {
        self.use_post = enabled;
    }

    pub fn use_post(&self) -> bool {
        self.use_post
    }

    pub fn resolve(&self, hostname: &str) -> Result<Vec<IpAddr>, String> {
        if !self.enabled {
            return Err("DoH resolver is disabled".into());
        }

        if hostname.is_empty() {
            return Err("Hostname cannot be empty".into());
        }

        Err("DoH network transport is not implemented".into())
    }
}
