use super::capability_manager::CapabilityManager;
use super::exploit_protection::{
    ExploitProtection,
    ExploitProtectionLevel,
};
use super::memory_safety::{
    MemoryProtection,
    MemoryProtectionLevel,
};
use super::privilege_manager::{
    PrivilegeLevel,
    PrivilegeManager,
};
use super::sandbox_security::{
    SandboxSecurity,
    SandboxSecurityLevel,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    Standard,
    Strict,
    Maximum,
    Lockdown,
    Custom,
}

#[derive(Debug)]
pub struct SecurityManager {
    pub level: SecurityLevel,
    pub exploit_protection: ExploitProtection,
    pub memory_safety: MemoryProtection,
    pub sandbox_security: SandboxSecurity,
    pub privilege_manager: PrivilegeManager,
    pub capability_manager: CapabilityManager,
}

impl SecurityManager {
    pub fn new() -> Self {
        let mut manager = Self {
            level: SecurityLevel::Strict,
            exploit_protection: ExploitProtection::new(),
            memory_safety: MemoryProtection::new(),
            sandbox_security: SandboxSecurity::new(),
            privilege_manager: PrivilegeManager::new(),
            capability_manager: CapabilityManager::new(),
        };

        manager.apply_level(SecurityLevel::Strict);
        manager
    }

    pub fn set_level(&mut self, level: SecurityLevel) {
        self.level = level;
        self.apply_level(level);
    }

    pub fn apply_level(&mut self, level: SecurityLevel) {
        match level {
            SecurityLevel::Standard => {
                self.exploit_protection
                    .set_level(ExploitProtectionLevel::Standard);

                self.memory_safety
                    .set_level(MemoryProtectionLevel::Standard);

                self.sandbox_security
                    .set_level(SandboxSecurityLevel::Standard);

                self.privilege_manager
                    .set_default_level(PrivilegeLevel::User);
            }

            SecurityLevel::Strict => {
                self.exploit_protection
                    .set_level(ExploitProtectionLevel::Strict);

                self.memory_safety
                    .set_level(MemoryProtectionLevel::Strict);

                self.sandbox_security
                    .set_level(SandboxSecurityLevel::Strict);

                self.privilege_manager
                    .set_default_level(PrivilegeLevel::Restricted);
            }

            SecurityLevel::Maximum => {
                self.exploit_protection
                    .set_level(ExploitProtectionLevel::Maximum);

                self.memory_safety
                    .set_level(MemoryProtectionLevel::Maximum);

                self.sandbox_security
                    .set_level(SandboxSecurityLevel::Maximum);

                self.privilege_manager
                    .set_default_level(PrivilegeLevel::Sandboxed);
            }

            SecurityLevel::Lockdown => {
                self.exploit_protection
                    .set_level(ExploitProtectionLevel::Lockdown);

                self.memory_safety
                    .set_level(MemoryProtectionLevel::Lockdown);

                self.sandbox_security
                    .set_level(SandboxSecurityLevel::Lockdown);

                self.privilege_manager
                    .set_default_level(PrivilegeLevel::Guest);

                self.capability_manager.revoke_all();
            }

            SecurityLevel::Custom => {}
        }
    }

    pub fn is_lockdown(&self) -> bool {
        self.level == SecurityLevel::Lockdown
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}
