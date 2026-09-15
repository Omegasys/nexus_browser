//! Tor network lock.
//!
//! Prevents traffic leaks when Tor fails.

pub struct TorNetworkLock {


    enabled: bool,

    tor_connected: bool,


}



impl TorNetworkLock {


    pub fn new() -> Self {


        Self {

            enabled:
                false,

            tor_connected:
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



    pub fn update_status(
        &mut self,
        connected:bool
    ) {

        self.tor_connected =
            connected;

    }



    pub fn allow_network(
        &self
    ) -> bool {


        if !self.enabled {

            return true;

        }


        self.tor_connected

    }

}


impl Default for TorNetworkLock {

    fn default() -> Self {

        Self::new()

    }

}
