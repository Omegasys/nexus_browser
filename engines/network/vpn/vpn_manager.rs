//! VPN management layer.
//!
//! Controls multiple VPN profiles.

use super::vpn_engine::{
    VpnEngine,
    VpnProtocol,
};



use std::collections::HashMap;



pub struct VpnManager {


    profiles:
        HashMap<String,VpnEngine>,


    active:
        Option<String>,


}



impl VpnManager {


    pub fn new() -> Self {


        Self {

            profiles:
                HashMap::new(),

            active:
                None,

        }

    }



    pub fn add_profile(
        &mut self,
        name: String,
        provider: String,
        protocol: VpnProtocol
    ) {


        let vpn =
            VpnEngine::new(
                provider,
                protocol
            );


        self.profiles
            .insert(
                name,
                vpn
            );

    }



    pub fn connect(
        &mut self,
        name: &str
    ) -> bool {


        if let Some(
            vpn
        ) =
            self.profiles
                .get_mut(name)
        {


            vpn.connect();


            self.active =
                Some(
                    name.to_string()
                );


            return true;

        }


        false

    }



    pub fn disconnect(
        &mut self
    ) {


        if let Some(
            name
        ) =
            self.active.take()
        {


            if let Some(
                vpn
            ) =
                self.profiles
                    .get_mut(&name)
            {

                vpn.disconnect();

            }

        }

    }



    pub fn active_profile(
        &self
    ) -> Option<&String> {

        self.active.as_ref()

    }

}


impl Default for VpnManager {


    fn default() -> Self {

        Self::new()

    }

}
