//! Core Lokinet engine.
//!
//! Controls Lokinet runtime lifecycle.

use super::tunnel::LokinetTunnel;
use super::route_manager::RouteManager;


#[derive(Debug, Clone)]

pub enum LokinetState {

    Stopped,

    Starting,

    Running,

    Failed,

}



pub struct LokinetEngine {


    state: LokinetState,

    tunnel: Option<LokinetTunnel>,

    routes: Option<RouteManager>,


}



impl LokinetEngine {


    pub fn new() -> Self {


        Self {

            state:
                LokinetState::Stopped,

            tunnel:
                None,

            routes:
                None,

        }

    }



    pub fn start(
        &mut self
    ) {


        self.state =
            LokinetState::Starting;


        self.tunnel =
            Some(
                LokinetTunnel::new()
            );


        self.routes =
            Some(
                RouteManager::new()
            );


        self.state =
            LokinetState::Running;

    }



    pub fn stop(
        &mut self
    ) {


        self.tunnel =
            None;


        self.routes =
            None;


        self.state =
            LokinetState::Stopped;

    }



    pub fn running(
        &self
    ) -> bool {


        matches!(
            self.state,
            LokinetState::Running
        )

    }



    pub fn state(
        &self
    ) -> &LokinetState {

        &self.state

    }

}



impl Default for LokinetEngine {


    fn default() -> Self {

        Self::new()

    }

}
