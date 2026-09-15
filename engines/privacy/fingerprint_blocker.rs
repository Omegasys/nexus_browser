//! Fingerprint protection system.
//!
//! Controls browser fingerprint exposure by normalizing
//! browser-identifying information.

use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct FingerprintProfile {

    pub name: String,

    pub user_agent: String,

    pub screen_resolution: String,

    pub timezone: String,

    pub language: String,

    pub hardware_concurrency: u32,

    pub canvas_protection: bool,

    pub webgl_protection: bool,

    pub audio_protection: bool,

}



impl Default for FingerprintProfile {

    fn default() -> Self {

        Self {

            name:
                "standard".to_string(),

            user_agent:
                "NexusBrowser".to_string(),

            screen_resolution:
                "1920x1080".to_string(),

            timezone:
                "UTC".to_string(),

            language:
                "en-US".to_string(),

            hardware_concurrency:
                4,

            canvas_protection:
                true,

            webgl_protection:
                true,

            audio_protection:
                true,

        }

    }

}



pub struct FingerprintBlocker {


    enabled: bool,

    active_profile: FingerprintProfile,

    overrides: HashMap<String, String>,


}



impl FingerprintBlocker {


    pub fn new() -> Self {

        Self {

            enabled: true,

            active_profile:
                FingerprintProfile::default(),

            overrides:
                HashMap::new(),

        }

    }



    pub fn enable(
        &mut self
    ) {

        self.enabled = true;

    }



    pub fn disable(
        &mut self
    ) {

        self.enabled = false;

    }



    pub fn is_enabled(
        &self
    ) -> bool {

        self.enabled

    }



    pub fn set_profile(
        &mut self,
        profile: FingerprintProfile
    ) {

        self.active_profile = profile;

    }



    pub fn add_override(
        &mut self,
        key: String,
        value: String
    ) {

        self.overrides.insert(
            key,
            value
        );

    }



    pub fn profile(
        &self
    ) -> &FingerprintProfile {

        &self.active_profile

    }

}


impl Default for FingerprintBlocker {

    fn default() -> Self {

        Self::new()

    }

}
