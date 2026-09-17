use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxSecurityLevel {
    Standard,
    Strict,
    Maximum,
    Lockdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandboxViolation {
    FilesystemAccess,
    NetworkAccess,
    ProcessCreation,
    PrivilegeEscalation,
    DeviceAccess,
    KernelInterface,
    SharedMemoryAccess,
    SandboxEscape,
    CapabilityViolation,
}

#[derive(Debug)]
pub struct SandboxSecurity {
    pub level: SandboxSecurityLevel,
    violations: HashSet<SandboxViolation>,
    terminated_on_violation: bool,
    network_isolated: bool,
    filesystem_isolated: bool,
    process_isolated: bool,
}

impl SandboxSecurity {
    pub fn new() -> Self {
        let mut security = Self {
            level: SandboxSecurityLevel::Strict,
            violations: HashSet::new(),
            terminated_on_violation: true,
            network_isolated: true,
            filesystem_isolated: true,
            process_isolated: true,
        };

        security.apply_level(SandboxSecurityLevel::Strict);
        security
    }

    pub fn set_level(&mut self, level: SandboxSecurityLevel) {
        self.level = level;
        self.apply_level(level);
    }

    pub fn apply_level(&mut self, level: SandboxSecurityLevel) {
        match level {
            SandboxSecurityLevel::Standard => {
                self.terminated_on_violation = false;
                self.network_isolated = false;
                self.filesystem_isolated = true;
                self.process_isolated = true;
            }

            SandboxSecurityLevel::Strict => {
                self.terminated_on_violation = true;
                self.network_isolated = true;
                self.filesystem_isolated = true;
                self.process_isolated = true;
            }

            SandboxSecurityLevel::Maximum => {
                self.terminated_on_violation = true;
                self.network_isolated = true;
                self.filesystem_isolated = true;
                self.process_isolated = true;
            }

            SandboxSecurityLevel::Lockdown => {
                self.terminated_on_violation = true;
                self.network_isolated = true;
                self.filesystem_isolated = true;
                self.process_isolated = true;
            }
        }
    }

    pub fn record_violation(&mut self, violation: SandboxViolation) {
        self.violations.insert(violation);
    }

    pub fn has_violation(&self, violation: SandboxViolation) -> bool {
        self.violations.contains(&violation)
    }

    pub fn should_terminate(&self) -> bool {
        self.terminated_on_violation && !self.violations.is_empty()
    }

    pub fn network_isolated(&self) -> bool {
        self.network_isolated
    }

    pub fn filesystem_isolated(&self) -> bool {
        self.filesystem_isolated
    }

    pub fn process_isolated(&self) -> bool {
        self.process_isolated
    }

    pub fn violations(&self) -> impl Iterator<Item = &SandboxViolation> {
        self.violations.iter()
    }

    pub fn clear_violations(&mut self) {
        self.violations.clear();
    }
}

impl Default for SandboxSecurity {
    fn default() -> Self {
        Self::new()
    }
}
