//! Core Tor engine.
//!
//! Controls Tor runtime lifecycle.

use super::tor_controller::TorController;


#[derive(Debug, Clone)]

pub enum TorState {

    Stopped,

    Starting,

    Running,

    Failed,

}



pub struct TorEngine {


    state: TorState,

    controller: Option<TorController>,


}



impl TorEngine {


    pub fn new() -> Self {


        Self {

            state:
                TorState::Stopped,

            controller:
                None,

        }

    }



    pub fn start(
        &mut self
    ) {


        self.state =
            TorState::Starting;


        self.controller =
            Some(
                TorController::new()
            );


        self.state =
            TorState::Running;

    }



    pub fn stop(
        &mut self
    ) {


        self.controller =
            None;


        self.state =
            TorState::Stopped;

    }



    pub fn running(
        &self
    ) -> bool {


        matches!(
            self.state,
            TorState::Running
        )

    }



    pub fn state(
        &self
    ) -> &TorState {

        &self.state

    }

}


impl Default for TorEngine {

    fn default() -> Self {

        Self::new()

    }

}
