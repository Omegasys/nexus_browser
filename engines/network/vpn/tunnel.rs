//! VPN encrypted tunnel abstraction.

#[derive(Debug, Clone)]

pub struct VpnTunnel {


    encrypted: bool,

    interface:
        Option<String>,


}



impl VpnTunnel {


    pub fn new() -> Self {


        Self {

            encrypted:
                true,

            interface:
                None,

        }

    }



    pub fn set_interface(
        &mut self,
        interface: String
    ) {


        self.interface =
            Some(interface);

    }



    pub fn encrypted(
        &self
    ) -> bool {

        self.encrypted

    }



    pub fn interface(
        &self
    ) -> Option<&String> {

        self.interface.as_ref()

    }



    pub fn close(
        &mut self
    ) {


        self.encrypted =
            false;


        self.interface =
            None;

    }

}



impl Default for VpnTunnel {


    fn default() -> Self {

        Self::new()

    }

}
