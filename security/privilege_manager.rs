use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrivilegeLevel {
    Guest,
    Sandboxed,
    Restricted,
    User,
    Elevated,
    Administrator,
    Kernel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeState {
    Active,
    Suspended,
    Revoked,
}

#[derive(Debug, Clone)]
pub struct PrivilegeContext {
    pub id: String,
    pub level: PrivilegeLevel,
    pub state: PrivilegeState,
}

#[derive(Debug)]
pub struct PrivilegeManager {
    default_level: PrivilegeLevel,
    contexts: HashMap<String, PrivilegeContext>,
}

impl PrivilegeManager {
    pub fn new() -> Self {
        Self {
            default_level: PrivilegeLevel::Restricted,
            contexts: HashMap::new(),
        }
    }

    pub fn set_default_level(&mut self, level: PrivilegeLevel) {
        self.default_level = level;
    }

    pub fn default_level(&self) -> PrivilegeLevel {
        self.default_level
    }

    pub fn create_context(&mut self, id: impl Into<String>) -> PrivilegeLevel {
        let id = id.into();

        let context = PrivilegeContext {
            id: id.clone(),
            level: self.default_level,
            state: PrivilegeState::Active,
        };

        self.contexts.insert(id, context);
        self.default_level
    }

    pub fn set_level(
        &mut self,
        id: &str,
        level: PrivilegeLevel,
    ) -> bool {
        let Some(context) = self.contexts.get_mut(id) else {
            return false;
        };

        context.level = level;
        true
    }

    pub fn suspend(&mut self, id: &str) -> bool {
        let Some(context) = self.contexts.get_mut(id) else {
            return false;
        };

        context.state = PrivilegeState::Suspended;
        true
    }

    pub fn revoke(&mut self, id: &str) -> bool {
        let Some(context) = self.contexts.get_mut(id) else {
            return false;
        };

        context.state = PrivilegeState::Revoked;
        true
    }

    pub fn get(&self, id: &str) -> Option<&PrivilegeContext> {
        self.contexts.get(id)
    }

    pub fn can_use(
        &self,
        id: &str,
        required_level: PrivilegeLevel,
    ) -> bool {
        let Some(context) = self.contexts.get(id) else {
            return false;
        };

        context.state == PrivilegeState::Active
            && context.level >= required_level
    }

    pub fn remove_context(&mut self, id: &str) -> bool {
        self.contexts.remove(id).is_some()
    }

    pub fn clear(&mut self) {
        self.contexts.clear();
    }
}

impl Default for PrivilegeManager {
    fn default() -> Self {
        Self::new()
    }
}
