#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceUiState {
    Creating,
    Active,
    Suspended,
    Locked,
    Archived,
}

#[derive(Debug, Clone)]
pub struct WorkspaceUiItem {
    pub id: String,
    pub name: String,
    pub state: WorkspaceUiState,
    pub tab_count: usize,
    pub active: bool,
    pub private: bool,
    pub isolated: bool,
    pub network_route: String,
    pub theme_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceUi {
    workspaces: Vec<WorkspaceUiItem>,
    active_workspace: Option<String>,
    show_workspace_switcher: bool,
    compact_mode: bool,
}

impl Default for WorkspaceUi {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceUi {
    pub fn new() -> Self {
        Self {
            workspaces: Vec::new(),
            active_workspace: None,
            show_workspace_switcher: true,
            compact_mode: false,
        }
    }

    pub fn workspaces(&self) -> &[WorkspaceUiItem] {
        &self.workspaces
    }

    pub fn add_workspace(&mut self, mut workspace: WorkspaceUiItem) {
        workspace.active = false;

        if self.workspaces.is_empty() {
            workspace.active = true;
            self.active_workspace = Some(workspace.id.clone());
        }

        self.workspaces.push(workspace);
    }

    pub fn remove_workspace(
        &mut self,
        workspace_id: &str,
    ) -> Option<WorkspaceUiItem> {
        let position = self
            .workspaces
            .iter()
            .position(|workspace| workspace.id == workspace_id)?;

        let removed = self.workspaces.remove(position);

        if self.active_workspace.as_deref() == Some(workspace_id) {
            self.active_workspace = self.workspaces.first().map(|workspace| {
                workspace.id.clone()
            });

            if let Some(active_id) = &self.active_workspace {
                for workspace in &mut self.workspaces {
                    workspace.active = workspace.id == *active_id;
                }
            }
        }

        Some(removed)
    }

    pub fn activate(
        &mut self,
        workspace_id: &str,
    ) -> Result<(), String> {
        let exists = self
            .workspaces
            .iter()
            .any(|workspace| workspace.id == workspace_id);

        if !exists {
            return Err(format!(
                "workspace '{workspace_id}' was not found"
            ));
        }

        self.active_workspace = Some(workspace_id.to_string());

        for workspace in &mut self.workspaces {
            workspace.active = workspace.id == workspace_id;
        }

        Ok(())
    }

    pub fn active(&self) -> Option<&WorkspaceUiItem> {
        let id = self.active_workspace.as_deref()?;

        self.workspaces
            .iter()
            .find(|workspace| workspace.id == id)
    }

    pub fn active_id(&self) -> Option<&str> {
        self.active_workspace.as_deref()
    }

    pub fn update_tab_count(
        &mut self,
        workspace_id: &str,
        tab_count: usize,
    ) {
        if let Some(workspace) = self
            .workspaces
            .iter_mut()
            .find(|workspace| workspace.id == workspace_id)
        {
            workspace.tab_count = tab_count;
        }
    }

    pub fn set_state(
        &mut self,
        workspace_id: &str,
        state: WorkspaceUiState,
    ) {
        if let Some(workspace) = self
            .workspaces
            .iter_mut()
            .find(|workspace| workspace.id == workspace_id)
        {
            workspace.state = state;
        }
    }

    pub fn set_theme(
        &mut self,
        workspace_id: &str,
        theme_id: Option<String>,
    ) {
        if let Some(workspace) = self
            .workspaces
            .iter_mut()
            .find(|workspace| workspace.id == workspace_id)
        {
            workspace.theme_id = theme_id;
        }
    }

    pub fn show_workspace_switcher(&self) -> bool {
        self.show_workspace_switcher
    }

    pub fn set_workspace_switcher(&mut self, visible: bool) {
        self.show_workspace_switcher = visible;
    }

    pub fn compact_mode(&self) -> bool {
        self.compact_mode
    }

    pub fn set_compact_mode(&mut self, enabled: bool) {
        self.compact_mode = enabled;
    }
}
