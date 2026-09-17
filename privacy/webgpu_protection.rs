#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebGpuProtectionMode {
    Allow,
    Block,
    Prompt,
    Restrict,
}

#[derive(Debug, Clone)]
pub struct WebGpuProtection {
    pub mode: WebGpuProtectionMode,
    pub expose_adapter_info: bool,
    pub expose_limits: bool,
    pub expose_features: bool,
    pub restrict_adapter_details: bool,
}

impl WebGpuProtection {
    pub fn new() -> Self {
        Self {
            mode: WebGpuProtectionMode::Restrict,
            expose_adapter_info: false,
            expose_limits: false,
            expose_features: false,
            restrict_adapter_details: true,
        }
    }

    pub fn set_mode(&mut self, mode: WebGpuProtectionMode) {
        self.mode = mode;
    }

    pub fn should_block(&self) -> bool {
        self.mode == WebGpuProtectionMode::Block
    }

    pub fn should_prompt(&self) -> bool {
        self.mode == WebGpuProtectionMode::Prompt
    }

    pub fn should_restrict(&self) -> bool {
        self.mode == WebGpuProtectionMode::Restrict
    }

    pub fn adapter_info_allowed(&self) -> bool {
        self.expose_adapter_info
    }

    pub fn limits_allowed(&self) -> bool {
        self.expose_limits
    }

    pub fn features_allowed(&self) -> bool {
        self.expose_features
    }
}

impl Default for WebGpuProtection {
    fn default() -> Self {
        Self::new()
    }
}
