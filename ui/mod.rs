pub mod browser_ui;
pub mod settings;
pub mod privacy_settings;
pub mod security_settings;
pub mod network_settings;

pub use browser_ui::{
    BrowserUi,
    BrowserUiState,
    NavigationBarState,
    SidebarState,
    StatusBarState,
    TabBarState,
};

pub use settings::{
    GeneralSettings,
    Settings,
    SettingsError,
    SettingsManager,
};

pub use privacy_settings::{
    FingerprintingProtection,
    PrivacySettings,
    TrackerProtection,
    WebRtcProtection,
};

pub use security_settings::{
    CertificateValidationMode,
    ContentSecurityPolicyMode,
    SecuritySettings,
    SandboxMode,
};

pub use network_settings::{
    DnsMode,
    NetworkRoute,
    NetworkSettings,
    ProxyMode,
};
pub mod dns_settings;
pub mod network_lock_ui;
pub mod engine_selector;
pub mod workspace_ui;
pub mod tab_ui;

pub use dns_settings::DnsSettings;

pub use network_lock_ui::{
    NetworkLockLayer,
    NetworkLockLayerState,
    NetworkLockUi,
    NetworkLockUiState,
};

pub use engine_selector::{
    EngineAvailability,
    EngineCategory,
    EngineOption,
    EngineSelector,
};

pub use workspace_ui::{
    WorkspaceUi,
    WorkspaceUiItem,
    WorkspaceUiState,
};

pub use tab_ui::{
    TabUi,
    TabUiItem,
    TabUiState,
};
