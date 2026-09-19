use std::collections::HashMap;
use std::time::SystemTime;

use super::isolation_model::WorkspaceIsolationPolicy;
use super::network_profiles::NetworkRoute;
use super::privacy_profiles::PrivacyProfileMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceState {
    Creating,
    Active,
    Suspended,
    Locked,
    Archived,
}

#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: u64,
    pub name: String,
    pub profile_id: Option<u64>,
    pub identity_id: Option<u64>,
    pub network_route: NetworkRoute,
    pub privacy_mode: PrivacyProfileMode,
    pub isolation: WorkspaceIsolationPolicy,
    pub state: WorkspaceState,
    pub created_at: SystemTime,
    pub last_used: SystemTime,
}

impl Workspace {
    pub fn new(
        id: u64,
        name: impl Into<String>,
    ) -> Self {
        let now = SystemTime::now();

        Self {
            id,
            name: name.into(),
            profile_id: None,
            identity_id: None,
            network_route: NetworkRoute::Direct,
            privacy_mode: PrivacyProfileMode::Standard,
            isolation: WorkspaceIsolationPolicy::strict(),
            state: WorkspaceState::Creating,
            created_at: now,
            last_used: now,
        }
    }

    pub fn activate(&mut self) {
        self.state = WorkspaceState::Active;
        self.last_used = SystemTime::now();
    }

    pub fn suspend(&mut self) {
        self.state = WorkspaceState::Suspended;
    }

    pub fn lock(&mut self) {
        self.state = WorkspaceState::Locked;
    }

    pub fn archive(&mut self) {
        self.state = WorkspaceState::Archived;
    }

    pub fn assign_profile(&mut self, profile_id: u64) {
        self.profile_id = Some(profile_id);
    }

    pub fn assign_identity(&mut self, identity_id: u64) {
        self.identity_id = Some(identity_id);
    }

    pub fn set_network_route(&mut self, route: NetworkRoute) {
        self.network_route = route;
    }

    pub fn set_privacy_mode(&mut self, mode: PrivacyProfileMode) {
        self.privacy_mode = mode;
    }

    pub fn is_active(&self) -> bool {
        self.state == WorkspaceState::Active
    }
}

#[derive(Debug, Default)]
pub struct WorkspaceManager {
    workspaces: HashMap<u64, Workspace>,
    active_workspace: Option<u64>,
    next_id: u64,
}

impl WorkspaceManager {
    pub fn new() -> Self {
        Self {
            workspaces: HashMap::new(),
            active_workspace: None,
            next_id: 1,
        }
    }

    pub fn create_workspace(
        &mut self,
        name: impl Into<String>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let mut workspace = Workspace::new(id, name);
        workspace.activate();

        self.workspaces.insert(id, workspace);

        if self.active_workspace.is_none() {
            self.active_workspace = Some(id);
        }

        id
    }

    pub fn get(&self, id: u64) -> Option<&Workspace> {
        self.workspaces.get(&id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut Workspace> {
        self.workspaces.get_mut(&id)
    }

    pub fn active(&self) -> Option<&Workspace> {
        self.active_workspace
            .and_then(|id| self.workspaces.get(&id))
    }

    pub fn active_mut(&mut self) -> Option<&mut Workspace> {
        let id = self.active_workspace?;
        self.workspaces.get_mut(&id)
    }

    pub fn active_id(&self) -> Option<u64> {
        self.active_workspace
    }

    pub fn activate_workspace(&mut self, id: u64) -> bool {
        if !self.workspaces.contains_key(&id) {
            return false;
        }

        if let Some(previous_id) = self.active_workspace {
            if previous_id != id {
                if let Some(previous) =
                    self.workspaces.get_mut(&previous_id)
                {
                    previous.suspend();
                }
            }
        }

        if let Some(workspace) = self.workspaces.get_mut(&id) {
            if workspace.state == WorkspaceState::Locked
                || workspace.state == WorkspaceState::Archived
            {
                return false;
            }

            workspace.activate();
            self.active_workspace = Some(id);
            return true;
        }

        false
    }

    pub fn suspend_workspace(&mut self, id: u64) -> bool {
        let Some(workspace) = self.workspaces.get_mut(&id) else {
            return false;
        };

        workspace.suspend();

        if self.active_workspace == Some(id) {
            self.active_workspace = None;
        }

        true
    }

    pub fn lock_workspace(&mut self, id: u64) -> bool {
        let Some(workspace) = self.workspaces.get_mut(&id) else {
            return false;
        };

        workspace.lock();

        if self.active_workspace == Some(id) {
            self.active_workspace = None;
        }

        true
    }

    pub fn archive_workspace(&mut self, id: u64) -> bool {
        let Some(workspace) = self.workspaces.get_mut(&id) else {
            return false;
        };

        workspace.archive();

        if self.active_workspace == Some(id) {
            self.active_workspace = None;
        }

        true
    }

    pub fn delete_workspace(
        &mut self,
        id: u64,
    ) -> Option<Workspace> {
        if self.active_workspace == Some(id) {
            self.active_workspace = None;
        }

        self.workspaces.remove(&id)
    }

    pub fn count(&self) -> usize {
        self.workspaces.len()
    }

    pub fn list(&self) -> Vec<&Workspace> {
        self.workspaces.values().collect()
    }
}
