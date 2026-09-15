//! Lokinet network lock.
//!
//! Prevents network leaks when
//! Lokinet becomes unavailable.

pub struct LokinetNetworkLock {


    enabled: bool,

    lokinet_connected: bool,


}



impl LokinetNetworkLock {


    pub fn new() -> Self {


        Self {

            enabled:
                false,

            lokinet_connected:
                false,

        }

    }



    pub fn enable(
        &mut self
    ) {

        self.enabled =
            true;

    }



    pub fn disable(
        &mut self
    ) {

        self.enabled =
            false;

    }



    pub fn update_state(
        &mut self,
        connected:bool
    ) {


        self.lokinet_connected =
            connected;

    }



    pub fn allow_network(
        &self
    ) -> bool {


        if !self.enabled {

            return true;

        }


        self.lokinet_connected

    }



    pub fn active(
        &self
    ) -> bool {

        self.enabled

    }

}



impl Default for LokinetNetworkLock {


    fn default() -> Self {

        Self::new()

    }

}
