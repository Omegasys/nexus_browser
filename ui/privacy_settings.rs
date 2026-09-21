#[derive(Debug, Clone)]
pub struct PrivacySettings {
    pub fingerprinting: FingerprintingProtection,
    pub tracker_protection: TrackerProtection,
    pub webrtc: WebRtcProtection,
    pub block_third_party_cookies: bool,
    pub partition_storage: bool,
    pub clear_data_on_exit: bool,
    pub clear_cookies_on_exit: bool,
    pub clear_cache_on_exit: bool,
    pub clear_site_data_on_exit: bool,
    pub private_browsing_default: bool,
    pub resist_link_tracking: bool,
    pub disable_prefetching: bool,
    pub disable_speculative_connections: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            fingerprinting: FingerprintingProtection::Standard,
            tracker_protection: TrackerProtection::Strict,
            webrtc: WebRtcProtection::BlockNonProxy,
            block_third_party_cookies: true,
            partition_storage: true,
            clear_data_on_exit: false,
            clear_cookies_on_exit: false,
            clear_cache_on_exit: false,
            clear_site_data_on_exit: false,
            private_browsing_default: false,
            resist_link_tracking: true,
            disable_prefetching: true,
            disable_speculative_connections: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FingerprintingProtection {
    Disabled,
    Standard,
    Strict,
    Maximum,
}

impl FingerprintingProtection {
    pub fn is_enabled(&self) -> bool {
        !matches!(self, Self::Disabled)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackerProtection {
    Disabled,
    Standard,
    Strict,
    Maximum,
    Custom,
}

impl TrackerProtection {
    pub fn is_enabled(&self) -> bool {
        !matches!(self, Self::Disabled)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebRtcProtection {
    Allow,
    Block,
    BlockNonProxy,
    ProxyOnly,
}

impl WebRtcProtection {
    pub fn blocks_direct_addresses(&self) -> bool {
        matches!(self, Self::Block | Self::BlockNonProxy | Self::ProxyOnly)
    }
}
