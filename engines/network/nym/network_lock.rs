//! Nym network lock.
//!
//! Prevents traffic leaks when
//! the Nym route is unavailable.

pub struct NymNetworkLock {


    enabled: bool,

    nym_connected: bool,


}



impl NymNetworkLock {


    pub fn new() -> Self {


        Self {

            enabled:
                false,

            nym_connected:
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
        connected: bool
    ) {


        self.nym_connected =
            connected;

    }



    pub fn allow_network(
        &self
    ) -> bool {


        if !self.enabled {

            return true;

        }


        self.nym_connected

    }



    pub fn active(
        &self
    ) -> bool {

        self.enabled

    }

}



impl Default for NymNetworkLock {


    fn default() -> Self {

        Self::new()

    }

}
