// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudProvider {
    None,
    OpenAI,
    Anthropic,
    Custom,
}

#[derive(Debug, Clone)]
pub struct CloudConfig {
    pub provider: CloudProvider,
    pub endpoint: String,
    pub model: String,
}

pub struct CloudBridge {
    enabled: bool,
    config: Option<CloudConfig>,
}

impl CloudBridge {
    pub fn new() -> Self {
        Self {
            enabled: false,
            config: None,
        }
    }

    pub fn configure(&mut self, config: CloudConfig) {
        self.config = Some(config);
    }

    pub fn enable(&mut self) -> Result<(), String> {
        if self.config.is_none() {
            return Err("Cloud AI provider is not configured".into());
        }

        self.enabled = true;
        Ok(())
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn send_prompt(&self, prompt: &str) -> Result<String, String> {
        if !self.enabled {
            return Err("Cloud AI bridge is disabled".into());
        }

        if prompt.is_empty() {
            return Err("Prompt cannot be empty".into());
        }

        // Placeholder for authenticated HTTPS API integration.
        Ok(format!("Cloud model response placeholder: {}", prompt))
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn config(&self) -> Option<&CloudConfig> {
        self.config.as_ref()
    }
}

impl Default for CloudBridge {
    fn default() -> Self {
        Self::new()
    }
}
