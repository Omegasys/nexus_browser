// SPDX-License-Identifier: GPL-3.0-or-later

pub mod privacy_manager;
pub mod fingerprinting;
pub mod tracking_protection;
pub mod cookie_control;
pub mod state_partitioning;
pub mod first_party_isolation;
pub mod referrer_control;

pub use privacy_manager::PrivacyManager;
pub use fingerprinting::FingerprintProtection;
pub use tracking_protection::TrackingProtection;
pub use cookie_control::CookieController;
pub use state_partitioning::StatePartitioning;
pub use first_party_isolation::FirstPartyIsolation;
pub use referrer_control::ReferrerController;
