#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsMode {
    Disabled,
    System,
    DoH,
    DoT,
    DnsCrypt,
    Tor,
}

#[derive(Debug, Clone)]
pub struct SecureDnsConfig {
    pub mode: DnsMode,
    pub allow_plaintext_fallback: bool,
    pub fail_closed: bool,
    pub validate_dnssec: bool,
    pub cache_enabled: bool,
    pub cache_size: usize,
}

impl Default for SecureDnsConfig {
    fn default() -> Self {
        Self {
            mode: DnsMode::DoH,
            allow_plaintext_fallback: false,
            fail_closed: true,
            validate_dnssec: true,
            cache_enabled: true,
            cache_size: 1024,
        }
    }
}

impl SecureDnsConfig {
    pub fn secure_default() -> Self {
        Self::default()
    }

    pub fn plaintext() -> Self {
        Self {
            mode: DnsMode::System,
            allow_plaintext_fallback: true,
            fail_closed: false,
            ..Self::default()
        }
    }

    pub fn doh() -> Self {
        Self {
            mode: DnsMode::DoH,
            allow_plaintext_fallback: false,
            fail_closed: true,
            ..Self::default()
        }
    }

    pub fn dot() -> Self {
        Self {
            mode: DnsMode::DoT,
            allow_plaintext_fallback: false,
            fail_closed: true,
            ..Self::default()
        }
    }

    pub fn dnscrypt() -> Self {
        Self {
            mode: DnsMode::DnsCrypt,
            allow_plaintext_fallback: false,
            fail_closed: true,
            ..Self::default()
        }
    }

    pub fn tor() -> Self {
        Self {
            mode: DnsMode::Tor,
            allow_plaintext_fallback: false,
            fail_closed: true,
            ..Self::default()
        }
    }

    pub fn is_secure(&self) -> bool {
        matches!(
            self.mode,
            DnsMode::DoH
                | DnsMode::DoT
                | DnsMode::DnsCrypt
                | DnsMode::Tor
        )
    }
}
