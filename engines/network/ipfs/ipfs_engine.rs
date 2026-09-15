//! Core IPFS engine.
//!
//! Controls IPFS runtime lifecycle.

use super::node_manager::IpfsNodeManager;


#[derive(Debug, Clone)]

pub enum IpfsState {

    Stopped,

    Starting,

    Running,

    Failed,

}



pub struct IpfsEngine {


    state: IpfsState,

    node:
        Option<IpfsNodeManager>,


}



impl IpfsEngine {


    pub fn new() -> Self {

        Self {

            state:
                IpfsState::Stopped,

            node:
                None,

        }

    }



    pub fn start(
        &mut self
    ) {


        self.state =
            IpfsState::Starting;


        self.node =
            Some(
                IpfsNodeManager::new()
            );


        self.state =
            IpfsState::Running;

    }



    pub fn stop(
        &mut self
    ) {


        self.node =
            None;


        self.state =
            IpfsState::Stopped;

    }



    pub fn running(
        &self
    ) -> bool {


        matches!(
            self.state,
            IpfsState::Running
        )

    }



    pub fn state(
        &self
    ) -> &IpfsState {

        &self.state

    }

}



impl Default for IpfsEngine {


    fn default() -> Self {

        Self::new()

    }

}
