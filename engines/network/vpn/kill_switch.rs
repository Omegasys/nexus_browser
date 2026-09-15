//! VPN network kill switch.
//!
//! Prevents traffic leakage when VPN fails.

#[derive(Debug, Clone)]

pub enum KillSwitchMode {

    Disabled,

    VPNOnly,

    AllNetwork,

    Strict,

}



pub struct VpnKillSwitch {


    enabled: bool,

    mode: KillSwitchMode,

    vpn_connected: bool,


}



impl VpnKillSwitch {


    pub fn new() -> Self {


        Self {

            enabled:
                false,

            mode:
                KillSwitchMode::Disabled,

            vpn_connected:
                false,

        }

    }



    pub fn enable(
        &mut self,
        mode: KillSwitchMode
    ) {


        self.enabled =
            true;


        self.mode =
            mode;

    }



    pub fn disable(
        &mut self
    ) {


        self.enabled =
            false;


        self.mode =
            KillSwitchMode::Disabled;

    }



    pub fn update_vpn_state(
        &mut self,
        connected: bool
    ) {


        self.vpn_connected =
            connected;

    }



    pub fn allow_network(
        &self
    ) -> bool {


        if !self.enabled {

            return true;

        }


        self.vpn_connected

    }



    pub fn active(
        &self
    ) -> bool {

        self.enabled

    }

}



impl Default for VpnKillSwitch {


    fn default() -> Self {

        Self::new()

    }

}
