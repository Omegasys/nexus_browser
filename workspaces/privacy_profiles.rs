use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyProfileMode {
    Standard,
    Private,
    Strict,
    Maximum,
    Custom,
}

#[derive(Debug, Clone)]
pub struct PrivacyProfile {
    pub name: String,
    pub mode: PrivacyProfileMode,

    pub block_third_party_cookies: bool,
    pub partition_storage: bool,
    pub partition_network_state: bool,

    pub fingerprinting_protection: bool,
    pub canvas_protection: bool,
    pub webgl_protection: bool,
    pub audio_fingerprinting_protection: bool,

    pub block_trackers: bool,
    pub block_known_ads: bool,
    pub block_social_trackers: bool,

    pub secure_dns: bool,
    pub prevent_dns_leaks: bool,
    pub prevent_web_rtc_leaks: bool,

    pub clear_session_data_on_close: bool,
    pub disable_persistent_identifiers: bool,
}

impl PrivacyProfile {
    pub fn standard() -> Self {
        Self {
            name: "Standard".to_string(),
            mode: PrivacyProfileMode::Standard,

            block_third_party_cookies: false,
            partition_storage: true,
            partition_network_state: true,

            fingerprinting_protection: false,
            canvas_protection: false,
            webgl_protection: false,
            audio_fingerprinting_protection: false,

            block_trackers: false,
            block_known_ads: false,
            block_social_trackers: false,

            secure_dns: true,
            prevent_dns_leaks: true,
            prevent_web_rtc_leaks: true,

            clear_session_data_on_close: false,
            disable_persistent_identifiers: false,
        }
    }

    pub fn private() -> Self {
        Self {
            name: "Private".to_string(),
            mode: PrivacyProfileMode::Private,

            block_third_party_cookies: true,
            partition_storage: true,
            partition_network_state: true,

            fingerprinting_protection: true,
            canvas_protection: true,
            webgl_protection: true,
            audio_fingerprinting_protection: true,

            block_trackers: true,
            block_known_ads: false,
            block_social_trackers: true,

            secure_dns: true,
            prevent_dns_leaks: true,
            prevent_web_rtc_leaks: true,

            clear_session_data_on_close: true,
            disable_persistent_identifiers: true,
        }
    }

    pub fn strict() -> Self {
        Self {
            name: "Strict".to_string(),
            mode: PrivacyProfileMode::Strict,

            block_third_party_cookies: true,
            partition_storage: true,
            partition_network_state: true,

            fingerprinting_protection: true,
            canvas_protection: true,
            webgl_protection: true,
            audio_fingerprinting_protection: true,

            block_trackers: true,
            block_known_ads: true,
            block_social_trackers: true,

            secure_dns: true,
            prevent_dns_leaks: true,
            prevent_web_rtc_leaks: true,

            clear_session_data_on_close: true,
            disable_persistent_identifiers: true,
        }
    }

    pub fn maximum() -> Self {
        Self {
            name: "Maximum".to_string(),
            mode: PrivacyProfileMode::Maximum,

            block_third_party_cookies: true,
            partition_storage: true,
            partition_network_state: true,

            fingerprinting_protection: true,
            canvas_protection: true,
            webgl_protection: true,
            audio_fingerprinting_protection: true,

            block_trackers: true,
            block_known_ads: true,
            block_social_trackers: true,

            secure_dns: true,
            prevent_dns_leaks: true,
            prevent_web_rtc_leaks: true,

            clear_session_data_on_close: true,
            disable_persistent_identifiers: true,
        }
    }

    pub fn custom(name: impl Into<String>) -> Self {
        let mut profile = Self::strict();

        profile.name = name.into();
        profile.mode = PrivacyProfileMode::Custom;

        profile
    }

    pub fn is_hardened(&self) -> bool {
        self.fingerprinting_protection
            && self.partition_storage
            && self.partition_network_state
            && self.prevent_dns_leaks
            && self.prevent_web_rtc_leaks
    }
}

#[derive(Debug, Default)]
pub struct PrivacyProfileManager {
    profiles: HashMap<String, PrivacyProfile>,
}

impl PrivacyProfileManager {
    pub fn new() -> Self {
        let mut manager = Self {
            profiles: HashMap::new(),
        };

        manager.register(PrivacyProfile::standard());
        manager.register(PrivacyProfile::private());
        manager.register(PrivacyProfile::strict());
        manager.register(PrivacyProfile::maximum());

        manager
    }

    pub fn register(&mut self, profile: PrivacyProfile) {
        self.profiles
            .insert(profile.name.clone(), profile);
    }

    pub fn get(
        &self,
        name: &str,
    ) -> Option<&PrivacyProfile> {
        self.profiles.get(name)
    }

    pub fn get_mut(
        &mut self,
        name: &str,
    ) -> Option<&mut PrivacyProfile> {
        self.profiles.get_mut(name)
    }

    pub fn remove(
        &mut self,
        name: &str,
    ) -> Option<PrivacyProfile> {
        self.profiles.remove(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.profiles.contains_key(name)
    }

    pub fn names(&self) -> Vec<String> {
        self.profiles.keys().cloned().collect()
    }

    pub fn count(&self) -> usize {
        self.profiles.len()
    }
}
