use super::secure_dns::DnsMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsPolicyMode {
    Standard,
    Strict,
    Maximum,
    Lockdown,
    Custom,
}

#[derive(Debug, Clone)]
pub struct DnsPolicy {
    pub mode: DnsPolicyMode,

    pub allow_system_dns: bool,
    pub allow_doh: bool,
    pub allow_dot: bool,
    pub allow_dnscrypt: bool,
    pub allow_tor_dns: bool,

    pub require_encryption: bool,
    pub require_dnssec: bool,
    pub allow_plaintext_fallback: bool,
    pub fail_closed: bool,

    pub block_private_resolvers: bool,
    pub prevent_resolver_bypass: bool,
}

impl Default for DnsPolicy {
    fn default() -> Self {
        Self::strict()
    }
}

impl DnsPolicy {
    pub fn standard() -> Self {
        Self {
            mode: DnsPolicyMode::Standard,
            allow_system_dns: true,
            allow_doh: true,
            allow_dot: true,
            allow_dnscrypt: true,
            allow_tor_dns: true,
            require_encryption: false,
            require_dnssec: true,
            allow_plaintext_fallback: true,
            fail_closed: false,
            block_private_resolvers: false,
            prevent_resolver_bypass: false,
        }
    }

    pub fn strict() -> Self {
        Self {
            mode: DnsPolicyMode::Strict,
            allow_system_dns: false,
            allow_doh: true,
            allow_dot: true,
            allow_dnscrypt: true,
            allow_tor_dns: true,
            require_encryption: true,
            require_dnssec: true,
            allow_plaintext_fallback: false,
            fail_closed: true,
            block_private_resolvers: false,
            prevent_resolver_bypass: true,
        }
    }

    pub fn maximum() -> Self {
        Self {
            mode: DnsPolicyMode::Maximum,
            allow_system_dns: false,
            allow_doh: true,
            allow_dot: true,
            allow_dnscrypt: true,
            allow_tor_dns: true,
            require_encryption: true,
            require_dnssec: true,
            allow_plaintext_fallback: false,
            fail_closed: true,
            block_private_resolvers: true,
            prevent_resolver_bypass: true,
        }
    }

    pub fn lockdown() -> Self {
        Self {
            mode: DnsPolicyMode::Lockdown,
            allow_system_dns: false,
            allow_doh: false,
            allow_dot: false,
            allow_dnscrypt: false,
            allow_tor_dns: false,
            require_encryption: true,
            require_dnssec: true,
            allow_plaintext_fallback: false,
            fail_closed: true,
            block_private_resolvers: true,
            prevent_resolver_bypass: true,
        }
    }

    pub fn allows(&self, mode: DnsMode) -> bool {
        match mode {
            DnsMode::Disabled => false,
            DnsMode::System => self.allow_system_dns,
            DnsMode::DoH => self.allow_doh,
            DnsMode::DoT => self.allow_dot,
            DnsMode::DnsCrypt => self.allow_dnscrypt,
            DnsMode::Tor => self.allow_tor_dns,
        }
    }

    pub fn validate_configuration(&self) -> Result<(), String> {
        if self.require_encryption && self.allow_system_dns {
            return Err(
                "Encrypted DNS is required but system DNS is allowed"
                    .into(),
            );
        }

        if !self.require_encryption && !self.allow_plaintext_fallback {
            return Err(
                "Plaintext fallback is disabled while encryption is optional"
                    .into(),
            );
        }

        Ok(())
    }
}
