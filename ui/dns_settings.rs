use super::network_settings::DnsMode;

#[derive(Debug, Clone)]
pub struct DnsSettings {
    pub mode: DnsMode,
    pub enabled: bool,
    pub require_encryption: bool,
    pub require_dnssec: bool,
    pub allow_system_resolver: bool,
    pub allow_plaintext_fallback: bool,
    pub fail_closed: bool,
    pub prevent_resolver_bypass: bool,
    pub cache_enabled: bool,
    pub cache_size: usize,
    pub partition_cache: bool,
    pub selected_resolver: Option<String>,
    pub fallback_resolver: Option<String>,
}

impl Default for DnsSettings {
    fn default() -> Self {
        Self {
            mode: DnsMode::DoH,
            enabled: true,
            require_encryption: true,
            require_dnssec: true,
            allow_system_resolver: false,
            allow_plaintext_fallback: false,
            fail_closed: true,
            prevent_resolver_bypass: true,
            cache_enabled: true,
            cache_size: 1024,
            partition_cache: true,
            selected_resolver: None,
            fallback_resolver: None,
        }
    }
}

impl DnsSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn secure_default() -> Self {
        Self::default()
    }

    pub fn lockdown() -> Self {
        Self {
            mode: DnsMode::DoH,
            enabled: true,
            require_encryption: true,
            require_dnssec: true,
            allow_system_resolver: false,
            allow_plaintext_fallback: false,
            fail_closed: true,
            prevent_resolver_bypass: true,
            cache_enabled: true,
            cache_size: 1024,
            partition_cache: true,
            selected_resolver: None,
            fallback_resolver: None,
        }
    }

    pub fn set_mode(&mut self, mode: DnsMode) {
        self.mode = mode;

        if matches!(mode, DnsMode::System) {
            self.require_encryption = false;
            self.allow_system_resolver = true;
        } else if !matches!(mode, DnsMode::Disabled) {
            self.allow_system_resolver = false;
        }
    }

    pub fn set_resolver(&mut self, resolver: impl Into<String>) {
        self.selected_resolver = Some(resolver.into());
    }

    pub fn clear_resolver(&mut self) {
        self.selected_resolver = None;
    }

    pub fn set_fallback_resolver(&mut self, resolver: impl Into<String>) {
        self.fallback_resolver = Some(resolver.into());
    }

    pub fn clear_fallback_resolver(&mut self) {
        self.fallback_resolver = None;
    }

    pub fn encryption_required(&self) -> bool {
        self.require_encryption
    }

    pub fn resolver_bypass_blocked(&self) -> bool {
        self.prevent_resolver_bypass
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.cache_enabled && self.cache_size == 0 {
            return Err("DNS cache size must be greater than zero".to_string());
        }

        if self.require_encryption && matches!(self.mode, DnsMode::System) {
            return Err(
                "encrypted DNS is required but system DNS is selected".to_string()
            );
        }

        if !self.allow_plaintext_fallback
            && matches!(self.mode, DnsMode::System)
            && self.fail_closed
        {
            return Err(
                "system DNS cannot be used with plaintext fallback disabled and fail-closed enabled"
                    .to_string(),
            );
        }

        Ok(())
    }
}
