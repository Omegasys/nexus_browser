//! IPFS node management.
//!
//! Controls local IPFS node lifecycle.

pub struct IpfsNodeManager {


    running: bool,

    repository:
        String,


}



impl IpfsNodeManager {


    pub fn new() -> Self {

        Self {

            running:
                false,

            repository:
                "nexus-ipfs".to_string(),

        }

    }



    pub fn start(
        &mut self
    ) {

        self.running =
            true;

    }



    pub fn stop(
        &mut self
    ) {

        self.running =
            false;

    }



    pub fn is_running(
        &self
    ) -> bool {

        self.running

    }



    pub fn repository(
        &self
    ) -> &str {

        &self.repository

    }

}



impl Default for IpfsNodeManager {


    fn default() -> Self {

        Self::new()

    }

}
