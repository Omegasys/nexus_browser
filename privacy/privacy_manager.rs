// SPDX-License-Identifier: GPL-3.0-or-later

use super::cookie_control::CookieController;
use super::fingerprinting::FingerprintProtection;
use super::first_party_isolation::FirstPartyIsolation;
use super::referrer_control::ReferrerController;
use super::state_partitioning::StatePartitioning;
use super::tracking_protection::TrackingProtection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyLevel {
    Standard,
    Strict,
    Maximum,
    Custom,
}

pub struct PrivacyManager {
    level: PrivacyLevel,
    fingerprinting: FingerprintProtection,
    tracking: TrackingProtection,
    cookies: CookieController,
    state_partitioning: StatePartitioning,
    first_party_isolation: FirstPartyIsolation,
    referrer: ReferrerController,
}

impl PrivacyManager {
    pub fn new() -> Self {
        Self {
            level: PrivacyLevel::Strict,
            fingerprinting: FingerprintProtection::new(),
            tracking: TrackingProtection::new(),
            cookies: CookieController::new(),
            state_partitioning: StatePartitioning::new(),
            first_party_isolation: FirstPartyIsolation::new(),
            referrer: ReferrerController::new(),
        }
    }

    pub fn apply_level(&mut self, level: PrivacyLevel) {
        self.level = level;

        match level {
            PrivacyLevel::Standard => {
                self.fingerprinting.set_enabled(true);
                self.tracking.set_enabled(true);
                self.cookies.set_third_party_blocking(true);
                self.state_partitioning.set_enabled(true);
                self.first_party_isolation.set_enabled(false);
                self.referrer.set_mode(
                    super::referrer_control::ReferrerMode::StrictOrigin,
                );
            }

            PrivacyLevel::Strict => {
                self.fingerprinting.set_enabled(true);
                self.tracking.set_enabled(true);
                self.cookies.set_third_party_blocking(true);
                self.state_partitioning.set_enabled(true);
                self.first_party_isolation.set_enabled(true);
                self.referrer.set_mode(
                    super::referrer_control::ReferrerMode::StrictOrigin,
                );
            }

            PrivacyLevel::Maximum => {
                self.fingerprinting.set_enabled(true);
                self.tracking.set_enabled(true);
                self.cookies.set_third_party_blocking(true);
                self.state_partitioning.set_enabled(true);
                self.first_party_isolation.set_enabled(true);
                self.referrer.set_mode(
                    super::referrer_control::ReferrerMode::NoReferrer,
                );
            }

            PrivacyLevel::Custom => {}
        }
    }

    pub fn level(&self) -> PrivacyLevel {
        self.level
    }

    pub fn fingerprinting(&self) -> &FingerprintProtection {
        &self.fingerprinting
    }

    pub fn fingerprinting_mut(&mut self) -> &mut FingerprintProtection {
        &mut self.fingerprinting
    }

    pub fn tracking(&self) -> &TrackingProtection {
        &self.tracking
    }

    pub fn tracking_mut(&mut self) -> &mut TrackingProtection {
        &mut self.tracking
    }

    pub fn cookies(&self) -> &CookieController {
        &self.cookies
    }

    pub fn cookies_mut(&mut self) -> &mut CookieController {
        &mut self.cookies
    }

    pub fn state_partitioning(&self) -> &StatePartitioning {
        &self.state_partitioning
    }

    pub fn state_partitioning_mut(&mut self) -> &mut StatePartitioning {
        &mut self.state_partitioning
    }

    pub fn first_party_isolation(&self) -> &FirstPartyIsolation {
        &self.first_party_isolation
    }

    pub fn first_party_isolation_mut(&mut self) -> &mut FirstPartyIsolation {
        &mut self.first_party_isolation
    }

    pub fn referrer(&self) -> &ReferrerController {
        &self.referrer
    }

    pub fn referrer_mut(&mut self) -> &mut ReferrerController {
        &mut self.referrer
    }
}

impl Default for PrivacyManager {
    fn default() -> Self {
        Self::new()
    }
}
