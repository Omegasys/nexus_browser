//! Nym gateway management.
//!
//! Handles connections into the Nym network.

use std::collections::HashMap;



#[derive(Debug, Clone)]

pub struct NymGateway {


    pub id: String,

    pub address: String,

    pub connected: bool,


}



pub struct GatewayManager {


    gateways:
        HashMap<String,NymGateway>,


}



impl NymGateway {


    pub fn new() -> Self {


        Self {

            id:
                "default".to_string(),

            address:
                "localhost".to_string(),

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

}



impl Default for NymGateway {


    fn default() -> Self {

        Self::new()

    }

}
