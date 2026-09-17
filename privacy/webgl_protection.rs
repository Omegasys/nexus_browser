#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebGlProtectionMode {
    Allow,
    Block,
    Prompt,
    Restrict,
}

#[derive(Debug, Clone)]
pub struct WebGlProtection {
    pub mode: WebGlProtectionMode,
    pub expose_debug_extensions: bool,
    pub expose_renderer: bool,
    pub expose_vendor: bool,
    pub restrict_extensions: bool,
}

impl WebGlProtection {
    pub fn new() -> Self {
        Self {
            mode: WebGlProtectionMode::Restrict,
            expose_debug_extensions: false,
            expose_renderer: false,
            expose_vendor: false,
            restrict_extensions: true,
        }
    }

    pub fn set_mode(&mut self, mode: WebGlProtectionMode) {
        self.mode = mode;
    }

    pub fn should_block(&self) -> bool {
        self.mode == WebGlProtectionMode::Block
    }

    pub fn should_prompt(&self) -> bool {
        self.mode == WebGlProtectionMode::Prompt
    }

    pub fn should_restrict(&self) -> bool {
        self.mode == WebGlProtectionMode::Restrict
    }

    pub fn renderer_allowed(&self) -> bool {
        self.expose_renderer
    }

    pub fn vendor_allowed(&self) -> bool {
        self.expose_vendor
    }

    pub fn debug_extensions_allowed(&self) -> bool {
        self.expose_debug_extensions
    }
}

impl Default for WebGlProtection {
    fn default() -> Self {
        Self::new()
    }
}
