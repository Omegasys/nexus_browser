//! Core I2P engine.
//!
//! Controls I2P runtime lifecycle.

use super::router::I2pRouter;


#[derive(Debug, Clone)]

pub enum I2pState {

    Stopped,

    Starting,

    Running,

    Failed,

}



pub struct I2pEngine {


    state: I2pState,

    router: Option<I2pRouter>,


}



impl I2pEngine {


    pub fn new() -> Self {


        Self {

            state:
                I2pState::Stopped,

            router:
                None,

        }

    }



    pub fn start(
        &mut self
    ) {


        self.state =
            I2pState::Starting;


        self.router =
            Some(
                I2pRouter::new()
            );


        self.state =
            I2pState::Running;

    }



    pub fn stop(
        &mut self
    ) {


        self.router =
            None;


        self.state =
            I2pState::Stopped;

    }



    pub fn running(
        &self
    ) -> bool {


        matches!(
            self.state,
            I2pState::Running
        )

    }



    pub fn state(
        &self
    ) -> &I2pState {

        &self.state

    }

}



impl Default for I2pEngine {


    fn default() -> Self {

        Self::new()

    }

}
