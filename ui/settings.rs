use super::network_settings::NetworkSettings;
use super::privacy_settings::PrivacySettings;
use super::security_settings::SecuritySettings;

#[derive(Debug, Clone)]
pub struct Settings {
    pub general: GeneralSettings,
    pub privacy: PrivacySettings,
    pub security: SecuritySettings,
    pub network: NetworkSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Settings {
    pub fn new() -> Self {
        Self {
            general: GeneralSettings::default(),
            privacy: PrivacySettings::default(),
            security: SecuritySettings::default(),
            network: NetworkSettings::default(),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone)]
pub struct GeneralSettings {
    pub homepage: String,
    pub search_engine: String,
    pub restore_previous_session: bool,
    pub open_new_tabs_with_homepage: bool,
    pub downloads_directory: String,
    pub ask_before_download: bool,
    pub language: String,
    pub spell_check: bool,
    pub hardware_acceleration: bool,
    pub smooth_scrolling: bool,
    pub autoplay_media: bool,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            homepage: "about:blank".to_string(),
            search_engine: "default".to_string(),
            restore_previous_session: false,
            open_new_tabs_with_homepage: false,
            downloads_directory: "Downloads".to_string(),
            ask_before_download: true,
            language: "en-US".to_string(),
            spell_check: true,
            hardware_acceleration: true,
            smooth_scrolling: true,
            autoplay_media: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct SettingsManager {
    settings: Settings,
    dirty: bool,
}

impl SettingsManager {
    pub fn new() -> Self {
        Self {
            settings: Settings::default(),
            dirty: false,
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        self.dirty = true;
        &mut self.settings
    }

    pub fn general(&self) -> &GeneralSettings {
        &self.settings.general
    }

    pub fn privacy(&self) -> &PrivacySettings {
        &self.settings.privacy
    }

    pub fn security(&self) -> &SecuritySettings {
        &self.settings.security
    }

    pub fn network(&self) -> &NetworkSettings {
        &self.settings.network
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub fn reset(&mut self) {
        self.settings.reset();
        self.dirty = true;
    }

    pub fn replace(&mut self, settings: Settings) {
        self.settings = settings;
        self.dirty = true;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsError {
    InvalidValue(String),
    Unsupported(String),
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidValue(message) => write!(formatter, "invalid setting: {message}"),
            Self::Unsupported(message) => write!(formatter, "unsupported setting: {message}"),
        }
    }
}

impl std::error::Error for SettingsError {}
