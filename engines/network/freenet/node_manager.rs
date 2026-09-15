//! Freenet node management.
//!
//! Controls local Freenet node lifecycle.

pub struct FreenetNodeManager {


    running: bool,

    storage_limit_mb: u64,


}



impl FreenetNodeManager {


    pub fn new() -> Self {


        Self {

            running:
                false,

            storage_limit_mb:
                10240,

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



    pub fn running(
        &self
    ) -> bool {

        self.running

    }



    pub fn storage_limit(
        &self
    ) -> u64 {

        self.storage_limit_mb

    }

}



impl Default for FreenetNodeManager {


    fn default() -> Self {

        Self::new()

    }

}
