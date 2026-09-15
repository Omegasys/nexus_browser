//! Core Freenet engine.
//!
//! Controls Freenet runtime lifecycle.

use super::node_manager::FreenetNodeManager;


#[derive(Debug, Clone)]

pub enum FreenetState {

    Stopped,

    Starting,

    Running,

    Failed,

}



pub struct FreenetEngine {


    state: FreenetState,

    node:
        Option<FreenetNodeManager>,


}



impl FreenetEngine {


    pub fn new() -> Self {


        Self {

            state:
                FreenetState::Stopped,

            node:
                None,

        }

    }



    pub fn start(
        &mut self
    ) {


        self.state =
            FreenetState::Starting;


        self.node =
            Some(
                FreenetNodeManager::new()
            );


        self.state =
            FreenetState::Running;

    }



    pub fn stop(
        &mut self
    ) {


        self.node =
            None;


        self.state =
            FreenetState::Stopped;

    }



    pub fn running(
        &self
    ) -> bool {


        matches!(
            self.state,
            FreenetState::Running
        )

    }



    pub fn state(
        &self
    ) -> &FreenetState {

        &self.state

    }

}



impl Default for FreenetEngine {


    fn default() -> Self {

        Self::new()

    }

}
