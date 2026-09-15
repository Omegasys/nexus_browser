//! Tor control interface.
//!
//! Handles communication with Tor daemon.

pub struct TorController {


    control_port: u16,

    connected: bool,


}



impl TorController {


    pub fn new() -> Self {


        Self {

            control_port:
                9051,

            connected:
                false,

        }

    }



    pub fn connect(
        &mut self
    ) {


        self.connected =
            true;

    }



    pub fn disconnect(
        &mut self
    ) {


        self.connected =
            false;

    }



    pub fn is_connected(
        &self
    ) -> bool {

        self.connected

    }



    pub fn control_port(
        &self
    ) -> u16 {

        self.control_port

    }

}


impl Default for TorController {

    fn default() -> Self {

        Self::new()

    }

}
