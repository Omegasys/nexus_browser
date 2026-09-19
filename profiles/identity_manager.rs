use std::collections::HashMap;
use std::time::SystemTime;

use super::identity_isolation::{
    IdentityIsolationPolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityState {
    Creating,
    Active,
    Suspended,
    Revoked,
    Destroyed,
}

#[derive(Debug, Clone)]
pub struct BrowserIdentity {
    pub id: u64,
    pub name: String,
    pub state: IdentityState,
    pub isolation: IdentityIsolationPolicy,
    pub created_at: SystemTime,
    pub last_used: SystemTime,
}

impl BrowserIdentity {
    pub fn new(
        id: u64,
        name: impl Into<String>,
        isolation: IdentityIsolationPolicy,
    ) -> Self {
        let now = SystemTime::now();

        Self {
            id,
            name: name.into(),
            state: IdentityState::Creating,
            isolation,
            created_at: now,
            last_used: now,
        }
    }

    pub fn activate(&mut self) {
        self.state = IdentityState::Active;
        self.last_used = SystemTime::now();
    }

    pub fn suspend(&mut self) {
        self.state = IdentityState::Suspended;
    }

    pub fn revoke(&mut self) {
        self.state = IdentityState::Revoked;
    }

    pub fn destroy(&mut self) {
        self.state = IdentityState::Destroyed;
    }

    pub fn is_active(&self) -> bool {
        self.state == IdentityState::Active
    }

    pub fn can_be_used(&self) -> bool {
        matches!(
            self.state,
            IdentityState::Creating | IdentityState::Active
        )
    }
}

#[derive(Debug, Default)]
pub struct IdentityManager {
    identities: HashMap<u64, BrowserIdentity>,
    active_identity: Option<u64>,
    next_id: u64,
}

impl IdentityManager {
    pub fn new() -> Self {
        Self {
            identities: HashMap::new(),
            active_identity: None,
            next_id: 1,
        }
    }

    pub fn create_identity(
        &mut self,
        name: impl Into<String>,
        isolation: IdentityIsolationPolicy,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let mut identity =
            BrowserIdentity::new(id, name, isolation);

        identity.activate();

        self.identities.insert(id, identity);

        if self.active_identity.is_none() {
            self.active_identity = Some(id);
        }

        id
    }

    pub fn get(&self, id: u64) -> Option<&BrowserIdentity> {
        self.identities.get(&id)
    }

    pub fn get_mut(
        &mut self,
        id: u64,
    ) -> Option<&mut BrowserIdentity> {
        self.identities.get_mut(&id)
    }

    pub fn active(&self) -> Option<&BrowserIdentity> {
        self.active_identity
            .and_then(|id| self.identities.get(&id))
    }

    pub fn active_id(&self) -> Option<u64> {
        self.active_identity
    }

    pub fn switch_identity(&mut self, id: u64) -> bool {
        let Some(identity) = self.identities.get(&id) else {
            return false;
        };

        if !identity.can_be_used() {
            return false;
        }

        if let Some(previous_id) = self.active_identity {
            if previous_id != id {
                if let Some(previous) =
                    self.identities.get_mut(&previous_id)
                {
                    previous.suspend();
                }
            }
        }

        if let Some(identity) = self.identities.get_mut(&id) {
            identity.activate();
            self.active_identity = Some(id);
            return true;
        }

        false
    }

    pub fn suspend_identity(&mut self, id: u64) -> bool {
        let Some(identity) = self.identities.get_mut(&id) else {
            return false;
        };

        identity.suspend();

        if self.active_identity == Some(id) {
            self.active_identity = None;
        }

        true
    }

    pub fn revoke_identity(&mut self, id: u64) -> bool {
        let Some(identity) = self.identities.get_mut(&id) else {
            return false;
        };

        identity.revoke();

        if self.active_identity == Some(id) {
            self.active_identity = None;
        }

        true
    }

    pub fn destroy_identity(&mut self, id: u64) -> bool {
        let Some(identity) = self.identities.get_mut(&id) else {
            return false;
        };

        identity.destroy();

        if self.active_identity == Some(id) {
            self.active_identity = None;
        }

        true
    }

    pub fn delete_identity(
        &mut self,
        id: u64,
    ) -> Option<BrowserIdentity> {
        if self.active_identity == Some(id) {
            self.active_identity = None;
        }

        self.identities.remove(&id)
    }

    pub fn count(&self) -> usize {
        self.identities.len()
    }
}
