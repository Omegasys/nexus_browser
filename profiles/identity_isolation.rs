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
    Storage,
    Network,
    Identity,
    StorageAndNetwork,
    Full,
}

#[derive(Debug, Clone)]
pub struct IdentityIsolationPolicy {
    pub level: IsolationLevel,
    pub boundary: IsolationBoundary,
    pub isolate_cookies: bool,
    pub isolate_storage: bool,
    pub isolate_cache: bool,
    pub isolate_network: bool,
    pub isolate_dns: bool,
    pub isolate_service_workers: bool,
    pub isolate_credentials: bool,
    pub isolate_fingerprinting_state: bool,
    pub prevent_cross_profile_access: bool,
}

impl Default for IdentityIsolationPolicy {
    fn default() -> Self {
        Self::strict()
    }
}

impl IdentityIsolationPolicy {
    pub fn none() -> Self {
        Self {
            level: IsolationLevel::None,
            boundary: IsolationBoundary::None,
            isolate_cookies: false,
            isolate_storage: false,
            isolate_cache: false,
            isolate_network: false,
            isolate_dns: false,
            isolate_service_workers: false,
            isolate_credentials: false,
            isolate_fingerprinting_state: false,
            prevent_cross_profile_access: false,
        }
    }

    pub fn basic() -> Self {
        Self {
            level: IsolationLevel::Basic,
            boundary: IsolationBoundary::Storage,
            isolate_cookies: true,
            isolate_storage: true,
            isolate_cache: false,
            isolate_network: false,
            isolate_dns: false,
            isolate_service_workers: true,
            isolate_credentials: true,
            isolate_fingerprinting_state: false,
            prevent_cross_profile_access: true,
        }
    }

    pub fn strict() -> Self {
        Self {
            level: IsolationLevel::Strict,
            boundary: IsolationBoundary::StorageAndNetwork,
            isolate_cookies: true,
            isolate_storage: true,
            isolate_cache: true,
            isolate_network: true,
            isolate_dns: true,
            isolate_service_workers: true,
            isolate_credentials: true,
            isolate_fingerprinting_state: true,
            prevent_cross_profile_access: true,
        }
    }

    pub fn maximum() -> Self {
        Self {
            level: IsolationLevel::Maximum,
            boundary: IsolationBoundary::Full,
            isolate_cookies: true,
            isolate_storage: true,
            isolate_cache: true,
            isolate_network: true,
            isolate_dns: true,
            isolate_service_workers: true,
            isolate_credentials: true,
            isolate_fingerprinting_state: true,
            prevent_cross_profile_access: true,
        }
    }

    pub fn allows_cross_profile_access(&self) -> bool {
        !self.prevent_cross_profile_access
    }

    pub fn isolates_storage(&self) -> bool {
        self.isolate_cookies
            || self.isolate_storage
            || self.isolate_cache
    }

    pub fn isolates_network(&self) -> bool {
        self.isolate_network || self.isolate_dns
    }

    pub fn isolates_identity(&self) -> bool {
        self.isolate_credentials
            || self.isolate_fingerprinting_state
    }

    pub fn should_isolate_service_workers(&self) -> bool {
        self.isolate_service_workers
    }
}
