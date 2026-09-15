//! IPv4 leak prevention.
//!
//! Used by VPN/Tor/privacy modes.

pub struct Ipv4LeakPrevention {


    enabled:bool,

    allowed_interface:
        Option<String>,


}



impl Ipv4LeakPrevention {


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



    pub fn interface(
        &self
    ) -> Option<&String> {

        self.allowed_interface
            .as_ref()

    }

}



impl Default for Ipv4LeakPrevention {

    fn default() -> Self {

        Self::new()

    }

}
