//! Lokinet LLARP tunnel abstraction.
//!
//! Handles encrypted Lokinet connections.

#[derive(Debug, Clone)]

pub struct LokinetTunnel {


    connected: bool,

    endpoint:
        Option<String>,


}



impl LokinetTunnel {


    pub fn new() -> Self {


        Self {

            connected:
                false,

            endpoint:
                None,

        }

    }



    pub fn connect(
        &mut self,
        endpoint:String
    ) {


        self.endpoint =
            Some(endpoint);


        self.connected =
            true;

    }



    pub fn disconnect(
        &mut self
    ) {


        self.endpoint =
            None;


        self.connected =
            false;

    }



    pub fn connected(
        &self
    ) -> bool {

        self.connected

    }



    pub fn endpoint(
        &self
    ) -> Option<&String> {

        self.endpoint.as_ref()

    }

}



impl Default for LokinetTunnel {


    fn default() -> Self {

        Self::new()

    }

}
