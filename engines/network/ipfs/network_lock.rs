//! IPFS network lock.
//!
//! Prevents accidental fallback
//! when IPFS-only mode is enabled.

pub struct IpfsNetworkLock {


    enabled: bool,

    ipfs_connected: bool,


}



impl IpfsNetworkLock {


    pub fn new() -> Self {

        Self {

            enabled:
                false,

            ipfs_connected:
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

        self.ipfs_connected =
            connected;

    }



    pub fn allow_network(
        &self
    ) -> bool {


        if !self.enabled {

            return true;

        }


        self.ipfs_connected

    }

}



impl Default for IpfsNetworkLock {

    fn default() -> Self {

        Self::new()

    }

}
