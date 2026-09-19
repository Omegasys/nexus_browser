use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct DotResolver {
    enabled: bool,
    server: String,
    port: u16,
    bootstrap_addresses: Vec<IpAddr>,
    verify_certificate: bool,
}

impl Default for DotResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl DotResolver {
    pub fn new() -> Self {
        Self {
            enabled: false,
            server: "dns.example.invalid".into(),
            port: 853,
            bootstrap_addresses: Vec::new(),
            verify_certificate: true,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_server(&mut self, server: impl Into<String>) {
        self.server = server.into();
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn add_bootstrap_address(&mut self, address: IpAddr) {
        if !self.bootstrap_addresses.contains(&address) {
            self.bootstrap_addresses.push(address);
        }
    }

    pub fn bootstrap_addresses(&self) -> &[IpAddr] {
        &self.bootstrap_addresses
    }

    pub fn set_certificate_verification(&mut self, enabled: bool) {
        self.verify_certificate = enabled;
    }

    pub fn certificate_verification(&self) -> bool {
        self.verify_certificate
    }

    pub fn resolve(&self, hostname: &str) -> Result<Vec<IpAddr>, String> {
        if !self.enabled {
            return Err("DoT resolver is disabled".into());
        }

        if hostname.is_empty() {
            return Err("Hostname cannot be empty".into());
        }

        if !self.verify_certificate {
            return Err(
                "DoT certificate verification must remain enabled".into(),
            );
        }

        Err("DoT network transport is not implemented".into())
    }
}
