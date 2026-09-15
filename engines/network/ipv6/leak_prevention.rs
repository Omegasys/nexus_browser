//! IPv6 leak prevention.
//!
//! Used for VPN/Tor/privacy profiles.

pub struct Ipv6LeakPrevention {


    enabled:bool,

    allowed_interface:
        Option<String>,


}



impl Ipv6LeakPrevention {


    pub fn new() -> Self {

        Self {

            enabled:false,

            allowed_interface:
                None,

        }

    }



    pub fn enable(
        &mut self,
        interface:String
    ) {

        self.enabled =
            true;

        self.allowed_interface =
            Some(interface);

    }



    pub fn disable(
        &mut self
    ) {

        self.enabled =
            false;

        self.allowed_interface =
            None;

    }



    pub fn active(
        &self
    ) -> bool {

        self.enabled

    }

}



impl Default for Ipv6LeakPrevention {

    fn default() -> Self {

        Self::new()

    }

}
