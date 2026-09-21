use std::collections::HashMap;

/// Controls the state of the main browser user interface.
#[derive(Debug, Clone)]
pub struct BrowserUi {
    state: BrowserUiState,
    navigation_bar: NavigationBarState,
    tab_bar: TabBarState,
    sidebar: SidebarState,
    status_bar: StatusBarState,
    fullscreen: bool,
    compact_mode: bool,
    workspace_id: Option<String>,
    window_title: String,
}

impl Default for BrowserUi {
    fn default() -> Self {
        Self::new()
    }
}

impl BrowserUi {
    pub fn new() -> Self {
        Self {
            state: BrowserUiState::Ready,
            navigation_bar: NavigationBarState::default(),
            tab_bar: TabBarState::default(),
            sidebar: SidebarState::default(),
            status_bar: StatusBarState::default(),
            fullscreen: false,
            compact_mode: false,
            workspace_id: None,
            window_title: "Nexus Browser".to_string(),
        }
    }

    pub fn state(&self) -> &BrowserUiState {
        &self.state
    }

    pub fn set_state(&mut self, state: BrowserUiState) {
        self.state = state;
    }

    pub fn navigation_bar(&self) -> &NavigationBarState {
        &self.navigation_bar
    }

    pub fn navigation_bar_mut(&mut self) -> &mut NavigationBarState {
        &mut self.navigation_bar
    }

    pub fn tab_bar(&self) -> &TabBarState {
        &self.tab_bar
    }

    pub fn tab_bar_mut(&mut self) -> &mut TabBarState {
        &mut self.tab_bar
    }

    pub fn sidebar(&self) -> &SidebarState {
        &self.sidebar
    }

    pub fn sidebar_mut(&mut self) -> &mut SidebarState {
        &mut self.sidebar
    }

    pub fn status_bar(&self) -> &StatusBarState {
        &self.status_bar
    }

    pub fn status_bar_mut(&mut self) -> &mut StatusBarState {
        &mut self.status_bar
    }

    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }

    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
    }

    pub fn compact_mode(&self) -> bool {
        self.compact_mode
    }

    pub fn set_compact_mode(&mut self, enabled: bool) {
        self.compact_mode = enabled;
    }

    pub fn workspace_id(&self) -> Option<&str> {
        self.workspace_id.as_deref()
    }

    pub fn set_workspace(&mut self, workspace_id: impl Into<String>) {
        self.workspace_id = Some(workspace_id.into());
    }

    pub fn clear_workspace(&mut self) {
        self.workspace_id = None;
    }

    pub fn window_title(&self) -> &str {
        &self.window_title
    }

    pub fn set_window_title(&mut self, title: impl Into<String>) {
        self.window_title = title.into();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserUiState {
    Starting,
    Ready,
    Loading,
    Busy,
    Suspended,
    Error,
}

#[derive(Debug, Clone)]
pub struct NavigationBarState {
    pub visible: bool,
    pub back_enabled: bool,
    pub forward_enabled: bool,
    pub reload_enabled: bool,
    pub home_enabled: bool,
    pub address_bar_visible: bool,
    pub bookmarks_button_visible: bool,
    pub downloads_button_visible: bool,
    pub extensions_button_visible: bool,
}

impl Default for NavigationBarState {
    fn default() -> Self {
        Self {
            visible: true,
            back_enabled: false,
            forward_enabled: false,
            reload_enabled: true,
            home_enabled: true,
            address_bar_visible: true,
            bookmarks_button_visible: true,
            downloads_button_visible: true,
            extensions_button_visible: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TabBarState {
    pub visible: bool,
    pub show_new_tab_button: bool,
    pub show_close_buttons: bool,
    pub show_tab_groups: bool,
    pub allow_dragging: bool,
    pub allow_reordering: bool,
    pub max_visible_tabs: usize,
}

impl Default for TabBarState {
    fn default() -> Self {
        Self {
            visible: true,
            show_new_tab_button: true,
            show_close_buttons: true,
            show_tab_groups: true,
            allow_dragging: true,
            allow_reordering: true,
            max_visible_tabs: 32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SidebarState {
    pub visible: bool,
    pub width: u32,
    pub bookmarks: bool,
    pub history: bool,
    pub downloads: bool,
    pub extensions: bool,
    pub workspaces: bool,
    pub developer_tools: bool,
}

impl Default for SidebarState {
    fn default() -> Self {
        Self {
            visible: false,
            width: 320,
            bookmarks: true,
            history: true,
            downloads: true,
            extensions: true,
            workspaces: true,
            developer_tools: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatusBarState {
    pub visible: bool,
    pub show_security_state: bool,
    pub show_network_route: bool,
    pub show_engine: bool,
    pub show_loading_progress: bool,
}

impl Default for StatusBarState {
    fn default() -> Self {
        Self {
            visible: false,
            show_security_state: true,
            show_network_route: true,
            show_engine: false,
            show_loading_progress: true,
        }
    }
}

/// Stores transient UI notifications.
#[derive(Debug, Clone)]
pub struct UiNotification {
    pub id: u64,
    pub title: String,
    pub message: String,
    pub severity: NotificationSeverity,
    pub persistent: bool,
}

/// Notification severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationSeverity {
    Info,
    Success,
    Warning,
    Error,
}

/// Lightweight notification manager.
#[derive(Debug, Default)]
pub struct NotificationManager {
    next_id: u64,
    notifications: HashMap<u64, UiNotification>,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            notifications: HashMap::new(),
        }
    }

    pub fn push(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        severity: NotificationSeverity,
        persistent: bool,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        self.notifications.insert(
            id,
            UiNotification {
                id,
                title: title.into(),
                message: message.into(),
                severity,
                persistent,
            },
        );

        id
    }

    pub fn remove(&mut self, id: u64) -> Option<UiNotification> {
        self.notifications.remove(&id)
    }

    pub fn get(&self, id: u64) -> Option<&UiNotification> {
        self.notifications.get(&id)
    }

    pub fn all(&self) -> impl Iterator<Item = &UiNotification> {
        self.notifications.values()
    }

    pub fn clear(&mut self) {
        self.notifications.clear();
    }

    pub fn count(&self) -> usize {
        self.notifications.len()
    }
}
