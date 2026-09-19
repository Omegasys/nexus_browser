use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabState {
    Creating,
    Loading,
    Active,
    Background,
    Suspended,
    Crashed,
    Closed,
}

#[derive(Debug, Clone)]
pub struct Tab {
    pub id: u64,
    pub workspace_id: u64,
    pub title: String,
    pub url: String,
    pub state: TabState,
    pub pinned: bool,
    pub muted: bool,
    pub group_id: Option<u64>,
    pub created_at: SystemTime,
    pub last_active: SystemTime,
}

impl Tab {
    pub fn new(
        id: u64,
        workspace_id: u64,
        url: impl Into<String>,
    ) -> Self {
        let now = SystemTime::now();

        Self {
            id,
            workspace_id,
            title: String::new(),
            url: url.into(),
            state: TabState::Creating,
            pinned: false,
            muted: false,
            group_id: None,
            created_at: now,
            last_active: now,
        }
    }

    pub fn set_url(&mut self, url: impl Into<String>) {
        self.url = url.into();
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn activate(&mut self) {
        self.state = TabState::Active;
        self.last_active = SystemTime::now();
    }

    pub fn set_loading(&mut self) {
        self.state = TabState::Loading;
    }

    pub fn background(&mut self) {
        self.state = TabState::Background;
    }

    pub fn suspend(&mut self) {
        self.state = TabState::Suspended;
    }

    pub fn crash(&mut self) {
        self.state = TabState::Crashed;
    }

    pub fn close(&mut self) {
        self.state = TabState::Closed;
    }

    pub fn set_pinned(&mut self, pinned: bool) {
        self.pinned = pinned;
    }

    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
    }

    pub fn set_group(&mut self, group_id: Option<u64>) {
        self.group_id = group_id;
    }

    pub fn is_open(&self) -> bool {
        self.state != TabState::Closed
    }
}

#[derive(Debug, Default)]
pub struct TabManager {
    tabs: HashMap<u64, Tab>,
    active_tabs: HashMap<u64, u64>,
    next_id: u64,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: HashMap::new(),
            active_tabs: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create_tab(
        &mut self,
        workspace_id: u64,
        url: impl Into<String>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let mut tab = Tab::new(id, workspace_id, url);
        tab.activate();

        self.tabs.insert(id, tab);
        self.active_tabs.insert(workspace_id, id);

        id
    }

    pub fn get(&self, id: u64) -> Option<&Tab> {
        self.tabs.get(&id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut Tab> {
        self.tabs.get_mut(&id)
    }

    pub fn active_tab(
        &self,
        workspace_id: u64,
    ) -> Option<&Tab> {
        self.active_tabs
            .get(&workspace_id)
            .and_then(|id| self.tabs.get(id))
    }

    pub fn active_tab_id(
        &self,
        workspace_id: u64,
    ) -> Option<u64> {
        self.active_tabs.get(&workspace_id).copied()
    }

    pub fn activate_tab(
        &mut self,
        workspace_id: u64,
        tab_id: u64,
    ) -> bool {
        let Some(tab) = self.tabs.get(&tab_id) else {
            return false;
        };

        if tab.workspace_id != workspace_id
            || !tab.is_open()
        {
            return false;
        }

        if let Some(previous_id) =
            self.active_tabs.get(&workspace_id).copied()
        {
            if previous_id != tab_id {
                if let Some(previous) =
                    self.tabs.get_mut(&previous_id)
                {
                    previous.background();
                }
            }
        }

        if let Some(tab) = self.tabs.get_mut(&tab_id) {
            tab.activate();
            self.active_tabs.insert(workspace_id, tab_id);
            return true;
        }

        false
    }

    pub fn tabs_for_workspace(
        &self,
        workspace_id: u64,
    ) -> Vec<&Tab> {
        self.tabs
            .values()
            .filter(|tab| {
                tab.workspace_id == workspace_id
                    && tab.is_open()
            })
            .collect()
    }

    pub fn close_tab(&mut self, id: u64) -> bool {
        let Some(tab) = self.tabs.get_mut(&id) else {
            return false;
        };

        let workspace_id = tab.workspace_id;
        tab.close();

        if self.active_tabs.get(&workspace_id) == Some(&id) {
            self.active_tabs.remove(&workspace_id);

            if let Some(next) = self
                .tabs_for_workspace(workspace_id)
                .into_iter()
                .next()
            {
                self.active_tabs.insert(
                    workspace_id,
                    next.id,
                );
            }
        }

        true
    }

    pub fn move_tab(
        &mut self,
        tab_id: u64,
        workspace_id: u64,
    ) -> bool {
        let Some(tab) = self.tabs.get_mut(&tab_id) else {
            return false;
        };

        let old_workspace = tab.workspace_id;
        tab.workspace_id = workspace_id;

        if self.active_tabs.get(&old_workspace) == Some(&tab_id) {
            self.active_tabs.remove(&old_workspace);
        }

        self.active_tabs.insert(workspace_id, tab_id);

        true
    }

    pub fn close_workspace_tabs(
        &mut self,
        workspace_id: u64,
    ) {
        let ids: Vec<u64> = self
            .tabs
            .values()
            .filter(|tab| tab.workspace_id == workspace_id)
            .map(|tab| tab.id)
            .collect();

        for id in ids {
            self.close_tab(id);
        }

        self.active_tabs.remove(&workspace_id);
    }

    pub fn count(&self) -> usize {
        self.tabs.len()
    }
}
