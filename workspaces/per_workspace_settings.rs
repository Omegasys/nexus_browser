use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WorkspaceSettings {
    pub workspace_id: u64,

    pub javascript_enabled: bool,
    pub images_enabled: bool,
    pub plugins_enabled: bool,
    pub popups_enabled: bool,
    pub autoplay_enabled: bool,
    pub web_rtc_enabled: bool,

    pub downloads_enabled: bool,
    pub notifications_enabled: bool,
    pub location_enabled: bool,
    pub camera_enabled: bool,
    pub microphone_enabled: bool,

    pub private_browsing: bool,
    pub clear_data_on_close: bool,
    pub restore_tabs_on_startup: bool,

    pub custom_user_agent: Option<String>,
    pub homepage: Option<String>,
}

impl Default for WorkspaceSettings {
    fn default() -> Self {
        Self {
            workspace_id: 0,

            javascript_enabled: true,
            images_enabled: true,
            plugins_enabled: true,
            popups_enabled: false,
            autoplay_enabled: false,
            web_rtc_enabled: false,

            downloads_enabled: true,
            notifications_enabled: false,
            location_enabled: false,
            camera_enabled: false,
            microphone_enabled: false,

            private_browsing: false,
            clear_data_on_close: false,
            restore_tabs_on_startup: true,

            custom_user_agent: None,
            homepage: None,
        }
    }
}

impl WorkspaceSettings {
    pub fn new(workspace_id: u64) -> Self {
        Self {
            workspace_id,
            ..Self::default()
        }
    }

    pub fn hardened(workspace_id: u64) -> Self {
        Self {
            workspace_id,

            javascript_enabled: true,
            images_enabled: true,
            plugins_enabled: false,
            popups_enabled: false,
            autoplay_enabled: false,
            web_rtc_enabled: false,

            downloads_enabled: true,
            notifications_enabled: false,
            location_enabled: false,
            camera_enabled: false,
            microphone_enabled: false,

            private_browsing: true,
            clear_data_on_close: true,
            restore_tabs_on_startup: false,

            custom_user_agent: None,
            homepage: None,
        }
    }

    pub fn set_homepage(
        &mut self,
        homepage: impl Into<String>,
    ) {
        self.homepage = Some(homepage.into());
    }

    pub fn set_user_agent(
        &mut self,
        user_agent: impl Into<String>,
    ) {
        self.custom_user_agent = Some(user_agent.into());
    }

    pub fn clear_user_agent(&mut self) {
        self.custom_user_agent = None;
    }
}

#[derive(Debug, Default)]
pub struct WorkspaceSettingsManager {
    settings: HashMap<u64, WorkspaceSettings>,
}

impl WorkspaceSettingsManager {
    pub fn new() -> Self {
        Self {
            settings: HashMap::new(),
        }
    }

    pub fn create(
        &mut self,
        workspace_id: u64,
    ) -> &mut WorkspaceSettings {
        self.settings
            .entry(workspace_id)
            .or_insert_with(|| WorkspaceSettings::new(workspace_id))
    }

    pub fn create_hardened(
        &mut self,
        workspace_id: u64,
    ) -> &mut WorkspaceSettings {
        self.settings
            .entry(workspace_id)
            .or_insert_with(|| WorkspaceSettings::hardened(workspace_id))
    }

    pub fn get(
        &self,
        workspace_id: u64,
    ) -> Option<&WorkspaceSettings> {
        self.settings.get(&workspace_id)
    }

    pub fn get_mut(
        &mut self,
        workspace_id: u64,
    ) -> Option<&mut WorkspaceSettings> {
        self.settings.get_mut(&workspace_id)
    }

    pub fn remove(
        &mut self,
        workspace_id: u64,
    ) -> Option<WorkspaceSettings> {
        self.settings.remove(&workspace_id)
    }

    pub fn clear_all(&mut self) {
        self.settings.clear();
    }

    pub fn count(&self) -> usize {
        self.settings.len()
    }
}
