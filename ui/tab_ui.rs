#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabUiState {
    Creating,
    Loading,
    Active,
    Background,
    Suspended,
    Crashed,
    Closed,
}

#[derive(Debug, Clone)]
pub struct TabUiItem {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub url: String,
    pub state: TabUiState,
    pub pinned: bool,
    pub muted: bool,
    pub audible: bool,
    pub loading_progress: Option<u8>,
    pub secure: bool,
    pub private: bool,
    pub group_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TabUi {
    tabs: Vec<TabUiItem>,
    active_tab: Option<String>,
    show_tab_groups: bool,
    allow_reordering: bool,
    allow_dragging: bool,
    show_close_buttons: bool,
}

impl Default for TabUi {
    fn default() -> Self {
        Self::new()
    }
}

impl TabUi {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab: None,
            show_tab_groups: true,
            allow_reordering: true,
            allow_dragging: true,
            show_close_buttons: true,
        }
    }

    pub fn tabs(&self) -> &[TabUiItem] {
        &self.tabs
    }

    pub fn add_tab(&mut self, mut tab: TabUiItem) {
        tab.state = if self.active_tab.is_none() {
            self.active_tab = Some(tab.id.clone());
            TabUiState::Active
        } else {
            TabUiState::Background
        };

        self.tabs.push(tab);
    }

    pub fn remove_tab(
        &mut self,
        tab_id: &str,
    ) -> Option<TabUiItem> {
        let position = self
            .tabs
            .iter()
            .position(|tab| tab.id == tab_id)?;

        let removed = self.tabs.remove(position);

        if self.active_tab.as_deref() == Some(tab_id) {
            self.active_tab = self.tabs.first().map(|tab| tab.id.clone());

            if let Some(active_id) = &self.active_tab {
                for tab in &mut self.tabs {
                    tab.state = if tab.id == *active_id {
                        TabUiState::Active
                    } else {
                        TabUiState::Background
                    };
                }
            }
        }

        Some(removed)
    }

    pub fn activate(
        &mut self,
        tab_id: &str,
    ) -> Result<(), String> {
        let exists = self.tabs.iter().any(|tab| tab.id == tab_id);

        if !exists {
            return Err(format!("tab '{tab_id}' was not found"));
        }

        self.active_tab = Some(tab_id.to_string());

        for tab in &mut self.tabs {
            if tab.id == tab_id {
                tab.state = TabUiState::Active;
            } else if tab.state != TabUiState::Closed {
                tab.state = TabUiState::Background;
            }
        }

        Ok(())
    }

    pub fn active(&self) -> Option<&TabUiItem> {
        let id = self.active_tab.as_deref()?;

        self.tabs.iter().find(|tab| tab.id == id)
    }

    pub fn active_id(&self) -> Option<&str> {
        self.active_tab.as_deref()
    }

    pub fn update_title(
        &mut self,
        tab_id: &str,
        title: impl Into<String>,
    ) {
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == tab_id) {
            tab.title = title.into();
        }
    }

    pub fn update_url(
        &mut self,
        tab_id: &str,
        url: impl Into<String>,
    ) {
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == tab_id) {
            tab.url = url.into();
        }
    }

    pub fn set_state(
        &mut self,
        tab_id: &str,
        state: TabUiState,
    ) {
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == tab_id) {
            tab.state = state;
        }
    }

    pub fn set_loading_progress(
        &mut self,
        tab_id: &str,
        progress: Option<u8>,
    ) {
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == tab_id) {
            tab.loading_progress = progress.map(|value| value.min(100));
        }
    }

    pub fn set_muted(
        &mut self,
        tab_id: &str,
        muted: bool,
    ) {
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == tab_id) {
            tab.muted = muted;
        }
    }

    pub fn set_pinned(
        &mut self,
        tab_id: &str,
        pinned: bool,
    ) {
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == tab_id) {
            tab.pinned = pinned;
        }
    }

    pub fn set_secure(
        &mut self,
        tab_id: &str,
        secure: bool,
    ) {
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == tab_id) {
            tab.secure = secure;
        }
    }

    pub fn assign_group(
        &mut self,
        tab_id: &str,
        group_id: Option<String>,
    ) {
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == tab_id) {
            tab.group_id = group_id;
        }
    }

    pub fn tabs_in_group(
        &self,
        group_id: &str,
    ) -> Vec<&TabUiItem> {
        self.tabs
            .iter()
            .filter(|tab| tab.group_id.as_deref() == Some(group_id))
            .collect()
    }

    pub fn reorder(
        &mut self,
        from: usize,
        to: usize,
    ) -> Result<(), String> {
        if !self.allow_reordering {
            return Err("tab reordering is disabled".to_string());
        }

        if from >= self.tabs.len() || to >= self.tabs.len() {
            return Err("tab index is out of range".to_string());
        }

        if from == to {
            return Ok(());
        }

        let tab = self.tabs.remove(from);
        self.tabs.insert(to, tab);

        Ok(())
    }

    pub fn set_show_tab_groups(&mut self, enabled: bool) {
        self.show_tab_groups = enabled;
    }

    pub fn show_tab_groups(&self) -> bool {
        self.show_tab_groups
    }

    pub fn set_allow_reordering(&mut self, enabled: bool) {
        self.allow_reordering = enabled;
    }

    pub fn allow_reordering(&self) -> bool {
        self.allow_reordering
    }

    pub fn set_allow_dragging(&mut self, enabled: bool) {
        self.allow_dragging = enabled;
    }

    pub fn allow_dragging(&self) -> bool {
        self.allow_dragging
    }

    pub fn set_show_close_buttons(&mut self, enabled: bool) {
        self.show_close_buttons = enabled;
    }

    pub fn show_close_buttons(&self) -> bool {
        self.show_close_buttons
    }

    pub fn count(&self) -> usize {
        self.tabs.len()
    }

    pub fn count_for_workspace(
        &self,
        workspace_id: &str,
    ) -> usize {
        self.tabs
            .iter()
            .filter(|tab| tab.workspace_id == workspace_id)
            .count()
    }
}
