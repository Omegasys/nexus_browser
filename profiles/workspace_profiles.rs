use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct WorkspaceProfile {
    pub id: u64,
    pub name: String,
    pub profile_ids: Vec<u64>,
    pub active_profile: Option<u64>,
    pub created_at: SystemTime,
}

impl WorkspaceProfile {
    pub fn new(
        id: u64,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            profile_ids: Vec::new(),
            active_profile: None,
            created_at: SystemTime::now(),
        }
    }

    pub fn add_profile(&mut self, profile_id: u64) {
        if !self.profile_ids.contains(&profile_id) {
            self.profile_ids.push(profile_id);
        }

        if self.active_profile.is_none() {
            self.active_profile = Some(profile_id);
        }
    }

    pub fn remove_profile(&mut self, profile_id: u64) -> bool {
        let original_len = self.profile_ids.len();

        self.profile_ids.retain(|id| *id != profile_id);

        if self.active_profile == Some(profile_id) {
            self.active_profile =
                self.profile_ids.first().copied();
        }

        original_len != self.profile_ids.len()
    }

    pub fn activate_profile(&mut self, profile_id: u64) -> bool {
        if !self.profile_ids.contains(&profile_id) {
            return false;
        }

        self.active_profile = Some(profile_id);
        true
    }

    pub fn contains_profile(&self, profile_id: u64) -> bool {
        self.profile_ids.contains(&profile_id)
    }
}

#[derive(Debug, Default)]
pub struct WorkspaceProfileManager {
    workspaces: HashMap<u64, WorkspaceProfile>,
    active_workspace: Option<u64>,
    next_id: u64,
}

impl WorkspaceProfileManager {
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

        let workspace = WorkspaceProfile::new(id, name);

        self.workspaces.insert(id, workspace);

        if self.active_workspace.is_none() {
            self.active_workspace = Some(id);
        }

        id
    }

    pub fn get(
        &self,
        id: u64,
    ) -> Option<&WorkspaceProfile> {
        self.workspaces.get(&id)
    }

    pub fn get_mut(
        &mut self,
        id: u64,
    ) -> Option<&mut WorkspaceProfile> {
        self.workspaces.get_mut(&id)
    }

    pub fn active(&self) -> Option<&WorkspaceProfile> {
        self.active_workspace
            .and_then(|id| self.workspaces.get(&id))
    }

    pub fn active_id(&self) -> Option<u64> {
        self.active_workspace
    }

    pub fn activate_workspace(&mut self, id: u64) -> bool {
        if !self.workspaces.contains_key(&id) {
            return false;
        }

        self.active_workspace = Some(id);
        true
    }

    pub fn add_profile(
        &mut self,
        workspace_id: u64,
        profile_id: u64,
    ) -> bool {
        let Some(workspace) =
            self.workspaces.get_mut(&workspace_id)
        else {
            return false;
        };

        workspace.add_profile(profile_id);
        true
    }

    pub fn remove_profile(
        &mut self,
        workspace_id: u64,
        profile_id: u64,
    ) -> bool {
        let Some(workspace) =
            self.workspaces.get_mut(&workspace_id)
        else {
            return false;
        };

        workspace.remove_profile(profile_id)
    }

    pub fn delete_workspace(
        &mut self,
        id: u64,
    ) -> Option<WorkspaceProfile> {
        if self.active_workspace == Some(id) {
            self.active_workspace = None;
        }

        self.workspaces.remove(&id)
    }

    pub fn count(&self) -> usize {
        self.workspaces.len()
    }
}
