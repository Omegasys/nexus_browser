use std::collections::HashMap;
use std::time::SystemTime;

use super::identity_manager::IdentityState;
use super::security_profiles::SecurityProfileMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileState {
    Creating,
    Active,
    Suspended,
    Locked,
    Archived,
}

#[derive(Debug, Clone)]
pub struct BrowserProfile {
    pub id: u64,
    pub name: String,
    pub identity_id: u64,
    pub security_mode: SecurityProfileMode,
    pub state: ProfileState,
    pub created_at: SystemTime,
    pub last_used: SystemTime,
}

impl BrowserProfile {
    pub fn new(
        id: u64,
        name: impl Into<String>,
        identity_id: u64,
        security_mode: SecurityProfileMode,
    ) -> Self {
        let now = SystemTime::now();

        Self {
            id,
            name: name.into(),
            identity_id,
            security_mode,
            state: ProfileState::Creating,
            created_at: now,
            last_used: now,
        }
    }

    pub fn activate(&mut self) {
        self.state = ProfileState::Active;
        self.last_used = SystemTime::now();
    }

    pub fn suspend(&mut self) {
        self.state = ProfileState::Suspended;
    }

    pub fn lock(&mut self) {
        self.state = ProfileState::Locked;
    }

    pub fn archive(&mut self) {
        self.state = ProfileState::Archived;
    }

    pub fn is_active(&self) -> bool {
        self.state == ProfileState::Active
    }

    pub fn can_start(&self) -> bool {
        matches!(
            self.state,
            ProfileState::Creating | ProfileState::Active
        )
    }
}

#[derive(Debug, Default)]
pub struct ProfileManager {
    profiles: HashMap<u64, BrowserProfile>,
    active_profile: Option<u64>,
    next_id: u64,
}

impl ProfileManager {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
            active_profile: None,
            next_id: 1,
        }
    }

    pub fn create_profile(
        &mut self,
        name: impl Into<String>,
        identity_id: u64,
        security_mode: SecurityProfileMode,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let mut profile = BrowserProfile::new(
            id,
            name,
            identity_id,
            security_mode,
        );

        profile.activate();

        self.profiles.insert(id, profile);

        if self.active_profile.is_none() {
            self.active_profile = Some(id);
        }

        id
    }

    pub fn get(&self, id: u64) -> Option<&BrowserProfile> {
        self.profiles.get(&id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut BrowserProfile> {
        self.profiles.get_mut(&id)
    }

    pub fn active(&self) -> Option<&BrowserProfile> {
        self.active_profile
            .and_then(|id| self.profiles.get(&id))
    }

    pub fn active_id(&self) -> Option<u64> {
        self.active_profile
    }

    pub fn activate_profile(&mut self, id: u64) -> bool {
        if !self.profiles.contains_key(&id) {
            return false;
        }

        if let Some(previous_id) = self.active_profile {
            if previous_id != id {
                if let Some(previous) = self.profiles.get_mut(&previous_id) {
                    previous.suspend();
                }
            }
        }

        if let Some(profile) = self.profiles.get_mut(&id) {
            if !profile.can_start() {
                return false;
            }

            profile.activate();
            self.active_profile = Some(id);
            return true;
        }

        false
    }

    pub fn suspend_profile(&mut self, id: u64) -> bool {
        let Some(profile) = self.profiles.get_mut(&id) else {
            return false;
        };

        profile.suspend();

        if self.active_profile == Some(id) {
            self.active_profile = None;
        }

        true
    }

    pub fn lock_profile(&mut self, id: u64) -> bool {
        let Some(profile) = self.profiles.get_mut(&id) else {
            return false;
        };

        profile.lock();

        if self.active_profile == Some(id) {
            self.active_profile = None;
        }

        true
    }

    pub fn archive_profile(&mut self, id: u64) -> bool {
        let Some(profile) = self.profiles.get_mut(&id) else {
            return false;
        };

        profile.archive();

        if self.active_profile == Some(id) {
            self.active_profile = None;
        }

        true
    }

    pub fn delete_profile(&mut self, id: u64) -> Option<BrowserProfile> {
        if self.active_profile == Some(id) {
            self.active_profile = None;
        }

        self.profiles.remove(&id)
    }

    pub fn profiles_for_identity(
        &self,
        identity_id: u64,
    ) -> Vec<&BrowserProfile> {
        self.profiles
            .values()
            .filter(|profile| profile.identity_id == identity_id)
            .collect()
    }

    pub fn count(&self) -> usize {
        self.profiles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }
}
