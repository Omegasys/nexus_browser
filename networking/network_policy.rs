use super::routing_manager::RouteTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkPolicyMode {
    Standard,
    Strict,
    Maximum,
    Lockdown,
    Custom,
}

#[derive(Debug, Clone)]
pub struct NetworkPolicy {
    pub mode: NetworkPolicyMode,
    pub allow_direct: bool,
    pub allow_proxy: bool,
    pub allow_vpn: bool,
    pub allow_tor: bool,
    pub allow_i2p: bool,
    pub allow_nym: bool,
    pub allow_lokinet: bool,
    pub allow_yggdrasil: bool,
    pub allow_ipfs: bool,
    pub allow_freenet: bool,
    pub allow_gnunet: bool,
    pub allow_hyphanet: bool,
    pub fail_closed: bool,
}

impl NetworkPolicy {
    pub fn new() -> Self {
        let mut policy = Self {
            mode: NetworkPolicyMode::Strict,
            allow_direct: true,
            allow_proxy: true,
            allow_vpn: true,
            allow_tor: true,
            allow_i2p: true,
            allow_nym: true,
            allow_lokinet: true,
            allow_yggdrasil: true,
            allow_ipfs: true,
            allow_freenet: true,
            allow_gnunet: true,
            allow_hyphanet: true,
            fail_closed: true,
        };

        policy.apply_mode(NetworkPolicyMode::Strict);
        policy
    }

    pub fn set_mode(&mut self, mode: NetworkPolicyMode) {
        self.mode = mode;
        self.apply_mode(mode);
    }

    pub fn apply_mode(&mut self, mode: NetworkPolicyMode) {
        match mode {
            NetworkPolicyMode::Standard => {
                self.allow_direct = true;
                self.allow_proxy = true;
                self.allow_vpn = true;
                self.allow_tor = true;
                self.allow_i2p = true;
                self.allow_nym = true;
                self.allow_lokinet = true;
                self.allow_yggdrasil = true;
                self.allow_ipfs = true;
                self.allow_freenet = true;
                self.allow_gnunet = true;
                self.allow_hyphanet = true;
                self.fail_closed = false;
            }

            NetworkPolicyMode::Strict => {
                self.allow_direct = true;
                self.allow_proxy = true;
                self.allow_vpn = true;
                self.allow_tor = true;
                self.allow_i2p = true;
                self.allow_nym = true;
                self.allow_lokinet = true;
                self.allow_yggdrasil = true;
                self.allow_ipfs = true;
                self.allow_freenet = true;
                self.allow_gnunet = true;
                self.allow_hyphanet = true;
                self.fail_closed = true;
            }

            NetworkPolicyMode::Maximum => {
                self.allow_direct = false;
                self.allow_proxy = true;
                self.allow_vpn = true;
                self.allow_tor = true;
                self.allow_i2p = true;
                self.allow_nym = true;
                self.allow_lokinet = true;
                self.allow_yggdrasil = true;
                self.allow_ipfs = true;
                self.allow_freenet = true;
                self.allow_gnunet = true;
                self.allow_hyphanet = true;
                self.fail_closed = true;
            }

            NetworkPolicyMode::Lockdown => {
                self.allow_direct = false;
                self.allow_proxy = false;
                self.allow_vpn = false;
                self.allow_tor = false;
                self.allow_i2p = false;
                self.allow_nym = false;
                self.allow_lokinet = false;
                self.allow_yggdrasil = false;
                self.allow_ipfs = false;
                self.allow_freenet = false;
                self.allow_gnunet = false;
                self.allow_hyphanet = false;
                self.fail_closed = true;
            }

            NetworkPolicyMode::Custom => {}
        }
    }

    pub fn allows(&self, target: RouteTarget) -> bool {
        match target {
            RouteTarget::Direct => self.allow_direct,
            RouteTarget::Proxy => self.allow_proxy,
            RouteTarget::Vpn => self.allow_vpn,
            RouteTarget::Tor => self.allow_tor,
            RouteTarget::I2p => self.allow_i2p,
            RouteTarget::Nym => self.allow_nym,
            RouteTarget::Lokinet => self.allow_lokinet,
            RouteTarget::Yggdrasil => self.allow_yggdrasil,
            RouteTarget::Ipfs => self.allow_ipfs,
            RouteTarget::Freenet => self.allow_freenet,
            RouteTarget::GnUnet => self.allow_gnunet,
            RouteTarget::Hyphanet => self.allow_hyphanet,
            RouteTarget::Blocked => false,
        }
    }
}

impl Default for NetworkPolicy {
    fn default() -> Self {
        Self::new()
    }
}
