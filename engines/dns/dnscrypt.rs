// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone)]
pub struct DnsCryptResolver {
    server_name: String,
    provider_name: String,
    enabled: bool,
    require_certificate_validation: bool,
}

impl DnsCryptResolver {
    pub fn new(
        server_name: impl Into<String>,
        provider_name: impl Into<String>,
    ) -> Self {
        Self {
            server_name: server_name.into(),
            provider_name: provider_name.into(),
            enabled: true,
            require_certificate_validation: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn set_certificate_validation(&mut self, enabled: bool) {
        self.require_certificate_validation = enabled;
    }

    pub fn server_name(&self) -> &str {
        &self.server_name
    }

    pub fn provider_name(&self) -> &str {
        &self.provider_name
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn certificate_validation_required(&self) -> bool {
        self.require_certificate_validation
    }

    pub fn resolve(&self, hostname: &str) -> Result<Vec<String>, String> {
        if !self.enabled {
            return Err("DNSCrypt resolver is disabled".into());
        }

        if !self.require_certificate_validation {
            return Err("DNSCrypt certificate validation is disabled".into());
        }

        if hostname.is_empty() {
            return Err("Hostname cannot be empty".into());
        }

        Ok(Vec::new())
    }
}
