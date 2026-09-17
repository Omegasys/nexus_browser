use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    Network,
    NetworkRaw,
    FilesystemRead,
    FilesystemWrite,
    ProcessSpawn,
    ProcessControl,
    DeviceAccess,
    Camera,
    Microphone,
    Location,
    ClipboardRead,
    ClipboardWrite,
    Storage,
    Cookies,
    Identity,
    Debug,
    Jit,
    Wasm,
    Gpu,
    Ipc,
    NativeMessaging,
    SystemConfiguration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityState {
    Granted,
    Denied,
    Revoked,
    Prompt,
}

#[derive(Debug)]
pub struct CapabilityManager {
    capabilities: HashMap<String, HashSet<Capability>>,
    states: HashMap<(String, Capability), CapabilityState>,
}

impl CapabilityManager {
    pub fn new() -> Self {
        Self {
            capabilities: HashMap::new(),
            states: HashMap::new(),
        }
    }

    pub fn grant(
        &mut self,
        context: impl Into<String>,
        capability: Capability,
    ) {
        let context = context.into();

        self.capabilities
            .entry(context.clone())
            .or_default()
            .insert(capability);

        self.states
            .insert((context, capability), CapabilityState::Granted);
    }

    pub fn deny(
        &mut self,
        context: impl Into<String>,
        capability: Capability,
    ) {
        let context = context.into();

        if let Some(capabilities) = self.capabilities.get_mut(&context) {
            capabilities.remove(&capability);
        }

        self.states
            .insert((context, capability), CapabilityState::Denied);
    }

    pub fn revoke(
        &mut self,
        context: impl Into<String>,
        capability: Capability,
    ) {
        let context = context.into();

        if let Some(capabilities) = self.capabilities.get_mut(&context) {
            capabilities.remove(&capability);
        }

        self.states
            .insert((context, capability), CapabilityState::Revoked);
    }

    pub fn set_prompt(
        &mut self,
        context: impl Into<String>,
        capability: Capability,
    ) {
        let context = context.into();

        if let Some(capabilities) = self.capabilities.get_mut(&context) {
            capabilities.remove(&capability);
        }

        self.states
            .insert((context, capability), CapabilityState::Prompt);
    }

    pub fn has(
        &self,
        context: &str,
        capability: Capability,
    ) -> bool {
        self.capabilities
            .get(context)
            .map(|set| set.contains(&capability))
            .unwrap_or(false)
    }

    pub fn state(
        &self,
        context: &str,
        capability: Capability,
    ) -> CapabilityState {
        self.states
            .get(&(context.to_string(), capability))
            .copied()
            .unwrap_or(CapabilityState::Denied)
    }

    pub fn capabilities_for(
        &self,
        context: &str,
    ) -> Vec<Capability> {
        self.capabilities
            .get(context)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default()
    }

    pub fn revoke_all(&mut self) {
        self.capabilities.clear();

        for state in self.states.values_mut() {
            *state = CapabilityState::Revoked;
        }
    }

    pub fn clear_context(&mut self, context: &str) {
        self.capabilities.remove(context);

        self.states
            .retain(|(stored_context, _), _| stored_context != context);
    }

    pub fn clear(&mut self) {
        self.capabilities.clear();
        self.states.clear();
    }
}

impl Default for CapabilityManager {
    fn default() -> Self {
        Self::new()
    }
}
