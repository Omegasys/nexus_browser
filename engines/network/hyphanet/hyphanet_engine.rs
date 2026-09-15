//! Core Hyphanet engine.
//!
//! Controls Hyphanet lifecycle.

use super::node_manager::HyphanetNodeManager;


#[derive(Debug, Clone)]

pub enum HyphanetState {

    Stopped,

    Starting,

    Running,

    Failed,

}



pub struct HyphanetEngine {


    state: HyphanetState,

    node:
        Option<HyphanetNodeManager>,


}



impl HyphanetEngine {


    pub fn new() -> Self {

        Self {

            state:
                HyphanetState::Stopped,

            node:
                None,

        }

    }



    pub fn start(
        &mut self
    ) {


        self.state =
            HyphanetState::Starting;


        self.node =
            Some(
                HyphanetNodeManager::new()
            );


        self.state =
            HyphanetState::Running;

    }



    pub fn stop(
        &mut self
    ) {


        self.node =
            None;


        self.state =
            HyphanetState::Stopped;

    }



    pub fn running(
        &self
    ) -> bool {


        matches!(
            self.state,
            HyphanetState::Running
        )

    }



    pub fn state(
        &self
    ) -> &HyphanetState {

        &self.state

    }

}



impl Default for HyphanetEngine {


    fn default() -> Self {

        Self::new()

    }

}
