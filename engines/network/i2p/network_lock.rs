//! I2P network lock.
//!
//! Prevents traffic leaks if I2P fails.

pub struct I2pNetworkLock {


    enabled: bool,

    i2p_connected: bool,


}



impl I2pNetworkLock {


    pub fn new() -> Self {


        Self {

            enabled:
                false,

            i2p_connected:
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


        self.i2p_connected =
            connected;

    }



    pub fn allow_network(
        &self
    ) -> bool {


        if !self.enabled {

            return true;

        }


        self.i2p_connected

    }



    pub fn active(
        &self
    ) -> bool {

        self.enabled

    }

}



impl Default for I2pNetworkLock {


    fn default() -> Self {

        Self::new()

    }

}
