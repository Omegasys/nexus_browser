//! Tor bridge management.
//!
//! Supports alternate entry points.

use std::collections::VecDeque;



pub struct BridgeManager {


    bridges:
        VecDeque<String>,


    enabled:
        bool,


}



impl BridgeManager {


    pub fn new() -> Self {


        Self {

            bridges:
                VecDeque::new(),

            enabled:
                false,

        }

    }



    pub fn add_bridge(
        &mut self,
        bridge: String
    ) {


        self.bridges
            .push_back(bridge);

    }



    pub fn remove_bridge(
        &mut self
    ) -> Option<String> {


        self.bridges
            .pop_front()

    }



    pub fn enable(
        &mut self
    ) {


        self.enabled =
            true;

    }



    pub fn enabled(
        &self
    ) -> bool {

        self.enabled

    }

}


impl Default for BridgeManager {

    fn default() -> Self {

        Self::new()

    }

}
