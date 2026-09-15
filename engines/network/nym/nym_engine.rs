//! Core Nym engine.
//!
//! Controls Nym runtime lifecycle.

use super::mixnet::Mixnet;
use super::gateway::NymGateway;


#[derive(Debug, Clone)]

pub enum NymState {

    Stopped,

    Starting,

    Running,

    Failed,

}



pub struct NymEngine {


    state: NymState,

    mixnet: Option<Mixnet>,

    gateway: Option<NymGateway>,


}



impl NymEngine {


    pub fn new() -> Self {


        Self {

            state:
                NymState::Stopped,

            mixnet:
                None,

            gateway:
                None,

        }

    }



    pub fn start(
        &mut self
    ) {


        self.state =
            NymState::Starting;


        self.mixnet =
            Some(
                Mixnet::new()
            );


        self.gateway =
            Some(
                NymGateway::new()
            );


        self.state =
            NymState::Running;

    }



    pub fn stop(
        &mut self
    ) {


        self.mixnet =
            None;


        self.gateway =
            None;


        self.state =
            NymState::Stopped;

    }



    pub fn running(
        &self
    ) -> bool {


        matches!(
            self.state,
            NymState::Running
        )

    }



    pub fn state(
        &self
    ) -> &NymState {

        &self.state

    }

}



impl Default for NymEngine {


    fn default() -> Self {

        Self::new()

    }

}
