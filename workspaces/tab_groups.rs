use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TabGroup {
    pub id: u64,
    pub workspace_id: u64,
    pub name: String,
    pub color: Option<String>,
    pub collapsed: bool,
    pub tab_ids: Vec<u64>,
}

impl TabGroup {
    pub fn new(
        id: u64,
        workspace_id: u64,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id,
            workspace_id,
            name: name.into(),
            color: None,
            collapsed: false,
            tab_ids: Vec::new(),
        }
    }

    pub fn add_tab(&mut self, tab_id: u64) {
        if !self.tab_ids.contains(&tab_id) {
            self.tab_ids.push(tab_id);
        }
    }

    pub fn remove_tab(&mut self, tab_id: u64) -> bool {
        let original_len = self.tab_ids.len();

        self.tab_ids.retain(|id| *id != tab_id);

        original_len != self.tab_ids.len()
    }

    pub fn set_color(
        &mut self,
        color: impl Into<String>,
    ) {
        self.color = Some(color.into());
    }

    pub fn clear_color(&mut self) {
        self.color = None;
    }

    pub fn toggle_collapsed(&mut self) {
        self.collapsed = !self.collapsed;
    }

    pub fn set_collapsed(&mut self, collapsed: bool) {
        self.collapsed = collapsed;
    }

    pub fn contains_tab(&self, tab_id: u64) -> bool {
        self.tab_ids.contains(&tab_id)
    }

    pub fn tab_count(&self) -> usize {
        self.tab_ids.len()
    }
}

#[derive(Debug, Default)]
pub struct TabGroupManager {
    groups: HashMap<u64, TabGroup>,
    next_id: u64,
}

impl TabGroupManager {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create_group(
        &mut self,
        workspace_id: u64,
        name: impl Into<String>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        self.groups.insert(
            id,
            TabGroup::new(id, workspace_id, name),
        );

        id
    }

    pub fn get(&self, id: u64) -> Option<&TabGroup> {
        self.groups.get(&id)
    }

    pub fn get_mut(
        &mut self,
        id: u64,
    ) -> Option<&mut TabGroup> {
        self.groups.get_mut(&id)
    }

    pub fn add_tab(
        &mut self,
        group_id: u64,
        tab_id: u64,
    ) -> bool {
        let Some(group) = self.groups.get_mut(&group_id) else {
            return false;
        };

        group.add_tab(tab_id);
        true
    }

    pub fn remove_tab(
        &mut self,
        group_id: u64,
        tab_id: u64,
    ) -> bool {
        let Some(group) = self.groups.get_mut(&group_id) else {
            return false;
        };

        group.remove_tab(tab_id)
    }

    pub fn groups_for_workspace(
        &self,
        workspace_id: u64,
    ) -> Vec<&TabGroup> {
        self.groups
            .values()
            .filter(|group| group.workspace_id == workspace_id)
            .collect()
    }

    pub fn delete_group(
        &mut self,
        id: u64,
    ) -> Option<TabGroup> {
        self.groups.remove(&id)
    }

    pub fn clear_workspace(
        &mut self,
        workspace_id: u64,
    ) {
        self.groups
            .retain(|_, group| group.workspace_id != workspace_id);
    }

    pub fn clear_all(&mut self) {
        self.groups.clear();
    }

    pub fn count(&self) -> usize {
        self.groups.len()
    }
}
