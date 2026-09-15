//! GNUnet network lock.
//!
//! Prevents fallback traffic
//! when GNUnet-only mode is enabled.

pub struct GnUnetNetworkLock {


    enabled:bool,

    connected:bool,


}



impl GnUnetNetworkLock {


    pub fn new() -> Self {


        Self {

            enabled:
                false,

            connected:
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

        self.connected =
            connected;

    }



    pub fn allow_network(
        &self
    ) -> bool {


        if !self.enabled {

            return true;

        }


        self.connected

    }

}



impl Default for GnUnetNetworkLock {

    fn default() -> Self {

        Self::new()

    }

}
