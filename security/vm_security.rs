use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmSecurityState {
    Creating,
    Starting,
    Running,
    Suspicious,
    Compromised,
    Quarantined,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmSecurityEvent {
    UnexpectedProcess,
    UnexpectedNetwork,
    FilesystemViolation,
    CapabilityViolation,
    EscapeAttempt,
    IntegrityFailure,
    PrivilegeEscalation,
}

#[derive(Debug, Clone)]
pub struct VmSecurityRecord {
    pub vm_id: String,
    pub state: VmSecurityState,
    pub isolated: bool,
    pub events: Vec<VmSecurityEvent>,
}

#[derive(Debug)]
pub struct VmSecurity {
    vms: HashMap<String, VmSecurityRecord>,
}

impl VmSecurity {
    pub fn new() -> Self {
        Self {
            vms: HashMap::new(),
        }
    }

    pub fn register(&mut self, vm_id: impl Into<String>) {
        let vm_id = vm_id.into();

        self.vms.insert(
            vm_id.clone(),
            VmSecurityRecord {
                vm_id,
                state: VmSecurityState::Creating,
                isolated: true,
                events: Vec::new(),
            },
        );
    }

    pub fn set_state(
        &mut self,
        vm_id: &str,
        state: VmSecurityState,
    ) -> bool {
        let Some(vm) = self.vms.get_mut(vm_id) else {
            return false;
        };

        vm.state = state;
        true
    }

    pub fn record_event(
        &mut self,
        vm_id: &str,
        event: VmSecurityEvent,
    ) -> bool {
        let Some(vm) = self.vms.get_mut(vm_id) else {
            return false;
        };

        vm.events.push(event);

        if matches!(
            event,
            VmSecurityEvent::EscapeAttempt
                | VmSecurityEvent::PrivilegeEscalation
                | VmSecurityEvent::IntegrityFailure
        ) {
            vm.state = VmSecurityState::Compromised;
        } else {
            vm.state = VmSecurityState::Suspicious;
        }

        true
    }

    pub fn quarantine(&mut self, vm_id: &str) -> bool {
        self.set_state(
            vm_id,
            VmSecurityState::Quarantined,
        )
    }

    pub fn terminate(&mut self, vm_id: &str) -> bool {
        self.set_state(
            vm_id,
            VmSecurityState::Terminated,
        )
    }

    pub fn is_compromised(&self, vm_id: &str) -> bool {
        self.vms
            .get(vm_id)
            .map(|vm| vm.state == VmSecurityState::Compromised)
            .unwrap_or(false)
    }

    pub fn get(&self, vm_id: &str) -> Option<&VmSecurityRecord> {
        self.vms.get(vm_id)
    }

    pub fn vms(&self) -> impl Iterator<Item = &VmSecurityRecord> {
        self.vms.values()
    }

    pub fn remove(&mut self, vm_id: &str) -> bool {
        self.vms.remove(vm_id).is_some()
    }
}

impl Default for VmSecurity {
    fn default() -> Self {
        Self::new()
    }
}
