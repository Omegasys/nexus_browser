//! IPFS gateway management.
//!
//! Allows HTTP access to IPFS resources.

use std::collections::VecDeque;



pub struct GatewayManager {


    gateways:
        VecDeque<String>,


}



impl GatewayManager {


    pub fn new() -> Self {

        Self {

            gateways:
                VecDeque::new(),

        }

    }



    pub fn add_gateway(
        &mut self,
        gateway:String
    ) {


        self.gateways
            .push_back(gateway);

    }



    pub fn gateway_count(
        &self
    ) -> usize {

        self.gateways.len()

    }



    pub fn first_gateway(
        &self
    ) -> Option<&String> {

        self.gateways.front()

    }

}



impl Default for GatewayManager {

    fn default() -> Self {

        Self::new()

    }

}
