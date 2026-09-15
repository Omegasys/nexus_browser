//! I2P router abstraction.
//!
//! Handles communication with the I2P router.

#[derive(Debug, Clone)]

pub struct I2pRouter {


    address: String,

    port: u16,

    connected: bool,


}



impl I2pRouter {


    pub fn new() -> Self {


        Self {

            address:
                "127.0.0.1".to_string(),

            port:
                7657,

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



    pub fn connected(
        &self
    ) -> bool {

        self.connected

    }



    pub fn address(
        &self
    ) -> &str {

        &self.address

    }



    pub fn port(
        &self
    ) -> u16 {

        self.port

    }

}



impl Default for I2pRouter {


    fn default() -> Self {

        Self::new()

    }

}
