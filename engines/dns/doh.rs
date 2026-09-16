// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone)]
pub struct DohResolver {
    endpoint: String,
    enabled: bool,
    bootstrap_address: Option<String>,
}

impl DohResolver {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            enabled: true,
            bootstrap_address: None,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn set_bootstrap_address(&mut self, address: impl Into<String>) {
        self.bootstrap_address = Some(address.into());
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn bootstrap_address(&self) -> Option<&str> {
        self.bootstrap_address.as_deref()
    }

    pub fn resolve(&self, hostname: &str) -> Result<Vec<String>, String> {
        if !self.enabled {
            return Err("DoH resolver is disabled".into());
        }

        if hostname.is_empty() {
            return Err("Hostname cannot be empty".into());
        }

        Ok(Vec::new())
    }
}
