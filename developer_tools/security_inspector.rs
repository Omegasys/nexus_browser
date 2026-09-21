use std::collections::HashMap;
use std::time::Instant;

/// Security subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecuritySubsystem {
    Sandboxing,
    SiteIsolation,
    ProcessIsolation,
    MicroVmIsolation,
    NetworkLock,
    KillSwitch,
    DnsProtection,
    CertificateValidation,
    ContentSecurityPolicy,
    ExtensionSandbox,
    PermissionSystem,
    StorageIsolation,
    FingerprintProtection,
    WebRtcProtection,
}

/// Security state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityState {
    Unknown,
    Enabled,
    Disabled,
    Degraded,
    Failed,
    Quarantined,
}

/// Security inspection record.
#[derive(Debug, Clone)]
pub struct SecurityComponent {
    pub subsystem: SecuritySubsystem,
    pub state: SecurityState,
    pub version: Option<String>,
    pub details: Option<String>,
    pub last_checked: Instant,
}

/// Security inspection result.
#[derive(Debug, Clone)]
pub struct SecurityInspectionResult {
    pub total_components: usize,
    pub enabled_components: usize,
    pub disabled_components: usize,
    pub degraded_components: usize,
    pub failed_components: usize,
    pub quarantined_components: usize,
    pub states: HashMap<SecuritySubsystem, SecurityState>,
}

/// Developer security inspector.
#[derive(Debug, Clone)]
pub struct SecurityInspector {
    components: HashMap<SecuritySubsystem, SecurityComponent>,
}

impl SecurityInspector {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        subsystem: SecuritySubsystem,
        state: SecurityState,
        version: Option<String>,
        details: Option<String>,
    ) {
        self.components.insert(
            subsystem,
            SecurityComponent {
                subsystem,
                state,
                version,
                details,
                last_checked: Instant::now(),
            },
        );
    }

    pub fn update_state(
        &mut self,
        subsystem: SecuritySubsystem,
        state: SecurityState,
    ) -> bool {
        if let Some(component) = self.components.get_mut(&subsystem) {
            component.state = state;
            component.last_checked = Instant::now();
            true
        } else {
            false
        }
    }

    pub fn get(
        &self,
        subsystem: SecuritySubsystem,
    ) -> Option<&SecurityComponent> {
        self.components.get(&subsystem)
    }

    pub fn components(
        &self,
    ) -> impl Iterator<Item = &SecurityComponent> {
        self.components.values()
    }

    pub fn inspect(&self) -> SecurityInspectionResult {
        let mut enabled = 0;
        let mut disabled = 0;
        let mut degraded = 0;
        let mut failed = 0;
        let mut quarantined = 0;

        let mut states = HashMap::new();

        for component in self.components.values() {
            states.insert(component.subsystem, component.state);

            match component.state {
                SecurityState::Enabled => enabled += 1,
                SecurityState::Disabled => disabled += 1,
                SecurityState::Degraded => degraded += 1,
                SecurityState::Failed => failed += 1,
                SecurityState::Quarantined => quarantined += 1,
                SecurityState::Unknown => {}
            }
        }

        SecurityInspectionResult {
            total_components: self.components.len(),
            enabled_components: enabled,
            disabled_components: disabled,
            degraded_components: degraded,
            failed_components: failed,
            quarantined_components: quarantined,
            states,
        }
    }

    pub fn security_failures(&self) -> Vec<&SecurityComponent> {
        self.components
            .values()
            .filter(|component| {
                matches!(
                    component.state,
                    SecurityState::Failed
                        | SecurityState::Degraded
                        | SecurityState::Quarantined
                )
            })
            .collect()
    }

    pub fn clear(&mut self) {
        self.components.clear();
    }
}

impl Default for SecurityInspector {
    fn default() -> Self {
        Self::new()
    }
}
