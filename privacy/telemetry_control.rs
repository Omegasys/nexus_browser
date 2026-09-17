use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelemetryMode {
    Disabled,
    EssentialOnly,
    OptIn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TelemetryCategory {
    Usage,
    Performance,
    CrashReports,
    Diagnostics,
    Security,
    Updates,
    Extensions,
    Search,
    Network,
}

#[derive(Debug, Clone)]
pub struct TelemetryControl {
    pub mode: TelemetryMode,
    enabled_categories: HashSet<TelemetryCategory>,
    allowed_endpoints: HashSet<String>,
}

impl TelemetryControl {
    pub fn new() -> Self {
        Self {
            mode: TelemetryMode::Disabled,
            enabled_categories: HashSet::new(),
            allowed_endpoints: HashSet::new(),
        }
    }

    pub fn set_mode(&mut self, mode: TelemetryMode) {
        self.mode = mode;

        if mode == TelemetryMode::Disabled {
            self.enabled_categories.clear();
        }
    }

    pub fn enable_category(&mut self, category: TelemetryCategory) {
        if self.mode != TelemetryMode::Disabled {
            self.enabled_categories.insert(category);
        }
    }

    pub fn disable_category(&mut self, category: TelemetryCategory) {
        self.enabled_categories.remove(&category);
    }

    pub fn allow_endpoint(&mut self, endpoint: impl Into<String>) {
        self.allowed_endpoints.insert(endpoint.into());
    }

    pub fn remove_endpoint(&mut self, endpoint: &str) {
        self.allowed_endpoints.remove(endpoint);
    }

    pub fn is_category_enabled(&self, category: TelemetryCategory) -> bool {
        match self.mode {
            TelemetryMode::Disabled => false,
            TelemetryMode::EssentialOnly => {
                matches!(
                    category,
                    TelemetryCategory::CrashReports
                        | TelemetryCategory::Diagnostics
                        | TelemetryCategory::Security
                        | TelemetryCategory::Updates
                )
            }
            TelemetryMode::OptIn => self.enabled_categories.contains(&category),
        }
    }

    pub fn is_endpoint_allowed(&self, endpoint: &str) -> bool {
        self.mode != TelemetryMode::Disabled
            && self.allowed_endpoints.contains(endpoint)
    }

    pub fn clear(&mut self) {
        self.enabled_categories.clear();
        self.allowed_endpoints.clear();
        self.mode = TelemetryMode::Disabled;
    }
}

impl Default for TelemetryControl {
    fn default() -> Self {
        Self::new()
    }
}
