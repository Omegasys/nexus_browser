use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityProfileMode {
    Standard,
    Private,
    Strict,
    Maximum,
    Custom,
}

#[derive(Debug, Clone)]
pub struct SecurityProfile {
    pub name: String,
    pub mode: SecurityProfileMode,

    pub javascript_enabled: bool,
    pub webassembly_enabled: bool,
    pub third_party_cookies: bool,
    pub persistent_storage: bool,
    pub service_workers: bool,
    pub web_rtc: bool,

    pub fingerprinting_protection: bool,
    pub network_isolation: bool,
    pub dns_protection: bool,
    pub leak_protection: bool,
    pub fail_closed: bool,

    pub downloads_enabled: bool,
    pub popups_enabled: bool,
    pub mixed_content: bool,
}

impl SecurityProfile {
    pub fn standard() -> Self {
        Self {
            name: "Standard".to_string(),
            mode: SecurityProfileMode::Standard,

            javascript_enabled: true,
            webassembly_enabled: true,
            third_party_cookies: true,
            persistent_storage: true,
            service_workers: true,
            web_rtc: true,

            fingerprinting_protection: false,
            network_isolation: false,
            dns_protection: false,
            leak_protection: false,
            fail_closed: false,

            downloads_enabled: true,
            popups_enabled: false,
            mixed_content: false,
        }
    }

    pub fn private() -> Self {
        Self {
            name: "Private".to_string(),
            mode: SecurityProfileMode::Private,

            javascript_enabled: true,
            webassembly_enabled: true,
            third_party_cookies: false,
            persistent_storage: true,
            service_workers: true,
            web_rtc: false,

            fingerprinting_protection: true,
            network_isolation: true,
            dns_protection: true,
            leak_protection: true,
            fail_closed: false,

            downloads_enabled: true,
            popups_enabled: false,
            mixed_content: false,
        }
    }

    pub fn strict() -> Self {
        Self {
            name: "Strict".to_string(),
            mode: SecurityProfileMode::Strict,

            javascript_enabled: true,
            webassembly_enabled: false,
            third_party_cookies: false,
            persistent_storage: true,
            service_workers: true,
            web_rtc: false,

            fingerprinting_protection: true,
            network_isolation: true,
            dns_protection: true,
            leak_protection: true,
            fail_closed: true,

            downloads_enabled: true,
            popups_enabled: false,
            mixed_content: false,
        }
    }

    pub fn maximum() -> Self {
        Self {
            name: "Maximum".to_string(),
            mode: SecurityProfileMode::Maximum,

            javascript_enabled: false,
            webassembly_enabled: false,
            third_party_cookies: false,
            persistent_storage: false,
            service_workers: false,
            web_rtc: false,

            fingerprinting_protection: true,
            network_isolation: true,
            dns_protection: true,
            leak_protection: true,
            fail_closed: true,

            downloads_enabled: false,
            popups_enabled: false,
            mixed_content: false,
        }
    }

    pub fn custom(name: impl Into<String>) -> Self {
        let mut profile = Self::strict();

        profile.name = name.into();
        profile.mode = SecurityProfileMode::Custom;

        profile
    }

    pub fn is_hardened(&self) -> bool {
        self.fingerprinting_protection
            && self.network_isolation
            && self.dns_protection
            && self.leak_protection
    }
}

#[derive(Debug, Default)]
pub struct SecurityProfileManager {
    profiles: HashMap<String, SecurityProfile>,
}

impl SecurityProfileManager {
    pub fn new() -> Self {
        let mut manager = Self {
            profiles: HashMap::new(),
        };

        manager.register(SecurityProfile::standard());
        manager.register(SecurityProfile::private());
        manager.register(SecurityProfile::strict());
        manager.register(SecurityProfile::maximum());

        manager
    }

    pub fn register(&mut self, profile: SecurityProfile) {
        self.profiles
            .insert(profile.name.clone(), profile);
    }

    pub fn get(&self, name: &str) -> Option<&SecurityProfile> {
        self.profiles.get(name)
    }

    pub fn get_mut(
        &mut self,
        name: &str,
    ) -> Option<&mut SecurityProfile> {
        self.profiles.get_mut(name)
    }

    pub fn remove(
        &mut self,
        name: &str,
    ) -> Option<SecurityProfile> {
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
