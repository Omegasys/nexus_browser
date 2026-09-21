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
