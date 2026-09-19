#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationLevel {
    None,
    Basic,
    Strict,
    Maximum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationBoundary {
    None,
    Tab,
    TabGroup,
    Workspace,
    WorkspaceAndIdentity,
    Full,
}

#[derive(Debug, Clone)]
pub struct WorkspaceIsolationPolicy {
    pub level: IsolationLevel,
    pub boundary: IsolationBoundary,

    pub isolate_tabs: bool,
    pub isolate_tab_groups: bool,
    pub isolate_storage: bool,
    pub isolate_cookies: bool,
    pub isolate_cache: bool,
    pub isolate_indexeddb: bool,
    pub isolate_service_workers: bool,
    pub isolate_network: bool,
    pub isolate_dns: bool,
    pub isolate_identity: bool,
    pub isolate_credentials: bool,
    pub isolate_fingerprinting_state: bool,

    pub allow_cross_workspace_tabs: bool,
    pub allow_cross_workspace_storage: bool,
    pub allow_cross_workspace_network: bool,
}

impl WorkspaceIsolationPolicy {
    pub fn none() -> Self {
        Self {
            level: IsolationLevel::None,
            boundary: IsolationBoundary::None,

            isolate_tabs: false,
            isolate_tab_groups: false,
            isolate_storage: false,
            isolate_cookies: false,
            isolate_cache: false,
            isolate_indexeddb: false,
            isolate_service_workers: false,
            isolate_network: false,
            isolate_dns: false,
            isolate_identity: false,
            isolate_credentials: false,
            isolate_fingerprinting_state: false,

            allow_cross_workspace_tabs: true,
            allow_cross_workspace_storage: true,
            allow_cross_workspace_network: true,
        }
    }

    pub fn basic() -> Self {
        Self {
            level: IsolationLevel::Basic,
            boundary: IsolationBoundary::Workspace,

            isolate_tabs: true,
            isolate_tab_groups: true,
            isolate_storage: true,
            isolate_cookies: true,
            isolate_cache: false,
            isolate_indexeddb: true,
            isolate_service_workers: true,
            isolate_network: false,
            isolate_dns: false,
            isolate_identity: false,
            isolate_credentials: true,
            isolate_fingerprinting_state: false,

            allow_cross_workspace_tabs: false,
            allow_cross_workspace_storage: false,
            allow_cross_workspace_network: true,
        }
    }

    pub fn strict() -> Self {
        Self {
            level: IsolationLevel::Strict,
            boundary: IsolationBoundary::WorkspaceAndIdentity,

            isolate_tabs: true,
            isolate_tab_groups: true,
            isolate_storage: true,
            isolate_cookies: true,
            isolate_cache: true,
            isolate_indexeddb: true,
            isolate_service_workers: true,
            isolate_network: true,
            isolate_dns: true,
            isolate_identity: true,
            isolate_credentials: true,
            isolate_fingerprinting_state: true,

            allow_cross_workspace_tabs: false,
            allow_cross_workspace_storage: false,
            allow_cross_workspace_network: false,
        }
    }

    pub fn maximum() -> Self {
        Self {
            level: IsolationLevel::Maximum,
            boundary: IsolationBoundary::Full,

            isolate_tabs: true,
            isolate_tab_groups: true,
            isolate_storage: true,
            isolate_cookies: true,
            isolate_cache: true,
            isolate_indexeddb: true,
            isolate_service_workers: true,
            isolate_network: true,
            isolate_dns: true,
            isolate_identity: true,
            isolate_credentials: true,
            isolate_fingerprinting_state: true,

            allow_cross_workspace_tabs: false,
            allow_cross_workspace_storage: false,
            allow_cross_workspace_network: false,
        }
    }

    pub fn is_storage_isolated(&self) -> bool {
        self.isolate_storage
            || self.isolate_cookies
            || self.isolate_cache
            || self.isolate_indexeddb
    }

    pub fn is_network_isolated(&self) -> bool {
        self.isolate_network || self.isolate_dns
    }

    pub fn is_identity_isolated(&self) -> bool {
        self.isolate_identity
            || self.isolate_credentials
            || self.isolate_fingerprinting_state
    }

    pub fn allows_cross_workspace_storage(&self) -> bool {
        self.allow_cross_workspace_storage
    }

    pub fn allows_cross_workspace_network(&self) -> bool {
        self.allow_cross_workspace_network
    }
}

impl Default for WorkspaceIsolationPolicy {
    fn default() -> Self {
        Self::strict()
    }
}
