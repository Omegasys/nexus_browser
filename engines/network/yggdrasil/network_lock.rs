//! Yggdrasil network lock.
//!
//! Prevents fallback traffic.

pub struct YggdrasilNetworkLock {


    enabled:bool,

    connected:bool,


}



impl YggdrasilNetworkLock {


    pub fn new() -> Self {

        Self {

            enabled:false,

            connected:false,

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



    pub fn update(
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



impl Default for YggdrasilNetworkLock {

    fn default() -> Self {

        Self::new()

    }

}
