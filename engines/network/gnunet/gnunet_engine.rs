//! Core GNUnet engine.
//!
//! Controls GNUnet runtime lifecycle.

use super::service_manager::ServiceManager;


#[derive(Debug, Clone)]

pub enum GnUnetState {

    Stopped,

    Starting,

    Running,

    Failed,

}



pub struct GnUnetEngine {


    state:
        GnUnetState,


    services:
        Option<ServiceManager>,


}



impl GnUnetEngine {


    pub fn new() -> Self {


        Self {

            state:
                GnUnetState::Stopped,

            services:
                None,

        }

    }



    pub fn start(
        &mut self
    ) {


        self.state =
            GnUnetState::Starting;


        self.services =
            Some(
                ServiceManager::new()
            );


        self.state =
            GnUnetState::Running;

    }



    pub fn stop(
        &mut self
    ) {


        self.services =
            None;


        self.state =
            GnUnetState::Stopped;

    }



    pub fn running(
        &self
    ) -> bool {


        matches!(
            self.state,
            GnUnetState::Running
        )

    }



    pub fn state(
        &self
    ) -> &GnUnetState {

        &self.state

    }

}



impl Default for GnUnetEngine {

    fn default() -> Self {

        Self::new()

    }

}
