#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebRtcMode {
    Allow,
    Block,
    Proxy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpHandlingPolicy {
    Default,
    DisableNonProxiedUdp,
    DisableUdp,
    ProxyOnly,
}

#[derive(Debug, Clone)]
pub struct WebRtcProtection {
    pub mode: WebRtcMode,
    pub ip_handling: IpHandlingPolicy,
    pub block_local_addresses: bool,
    pub block_public_addresses: bool,
}

impl WebRtcProtection {
    pub fn new() -> Self {
        Self {
            mode: WebRtcMode::Proxy,
            ip_handling: IpHandlingPolicy::ProxyOnly,
            block_local_addresses: true,
            block_public_addresses: false,
        }
    }

    pub fn set_mode(&mut self, mode: WebRtcMode) {
        self.mode = mode;
    }

    pub fn set_ip_handling(&mut self, policy: IpHandlingPolicy) {
        self.ip_handling = policy;
    }

    pub fn should_block(&self) -> bool {
        self.mode == WebRtcMode::Block
    }

    pub fn should_proxy(&self) -> bool {
        self.mode == WebRtcMode::Proxy
    }

    pub fn should_block_local_address(&self) -> bool {
        self.block_local_addresses
    }

    pub fn should_block_public_address(&self) -> bool {
        self.block_public_addresses
    }
}

impl Default for WebRtcProtection {
    fn default() -> Self {
        Self::new()
    }
}
