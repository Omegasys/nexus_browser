#[derive(Debug, Clone)]
pub struct SecuritySettings {
    pub sandbox: SandboxMode,
    pub certificate_validation: CertificateValidationMode,
    pub content_security_policy: ContentSecurityPolicyMode,
    pub site_isolation: bool,
    pub process_isolation: bool,
    pub microvm_isolation: bool,
    pub escape_detection: bool,
    pub network_lock: bool,
    pub kill_switch: bool,
    pub secure_dns_required: bool,
    pub block_mixed_content: bool,
    pub block_dangerous_downloads: bool,
    pub extension_sandbox: bool,
    pub strict_permissions: bool,
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            sandbox: SandboxMode::Strict,
            certificate_validation: CertificateValidationMode::Strict,
            content_security_policy: ContentSecurityPolicyMode::Enforce,
            site_isolation: true,
            process_isolation: true,
            microvm_isolation: true,
            escape_detection: true,
            network_lock: true,
            kill_switch: true,
            secure_dns_required: true,
            block_mixed_content: true,
            block_dangerous_downloads: true,
            extension_sandbox: true,
            strict_permissions: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxMode {
    Disabled,
    Standard,
    Strict,
    Maximum,
}

impl SandboxMode {
    pub fn is_enabled(&self) -> bool {
        !matches!(self, Self::Disabled)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateValidationMode {
    Standard,
    Strict,
    Maximum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentSecurityPolicyMode {
    Disabled,
    ReportOnly,
    Enforce,
    Strict,
}
