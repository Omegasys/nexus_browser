// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone)]
pub struct DotResolver {
    server: String,
    port: u16,
    enabled: bool,
    hostname_verification: bool,
}

impl DotResolver {
    pub fn new(server: impl Into<String>) -> Self {
        Self {
            server: server.into(),
            port: 853,
            enabled: true,
            hostname_verification: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn set_hostname_verification(&mut self, enabled: bool) {
        self.hostname_verification = enabled;
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn hostname_verification(&self) -> bool {
        self.hostname_verification
    }

    pub fn resolve(&self, hostname: &str) -> Result<Vec<String>, String> {
        if !self.enabled {
            return Err("DoT resolver is disabled".into());
        }

        if !self.hostname_verification {
            return Err("TLS hostname verification is disabled".into());
        }

        if hostname.is_empty() {
            return Err("Hostname cannot be empty".into());
        }

        Ok(Vec::new())
    }
}
