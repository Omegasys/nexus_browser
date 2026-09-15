//! Core Yggdrasil engine.
//!
//! Controls encrypted mesh lifecycle.

use super::node_manager::YggdrasilNodeManager;


#[derive(Debug,Clone)]

pub enum YggdrasilState {

    Stopped,

    Starting,

    Running,

    Failed,

}



pub struct YggdrasilEngine {


    state:YggdrasilState,

    node:
        Option<YggdrasilNodeManager>,


}



impl YggdrasilEngine {


    pub fn new() -> Self {

        Self {

            state:
                YggdrasilState::Stopped,

            node:
                None,

        }

    }



    pub fn start(
        &mut self
    ) {


        self.state =
            YggdrasilState::Starting;


        self.node =
            Some(
                YggdrasilNodeManager::new()
            );


        self.state =
            YggdrasilState::Running;

    }



    pub fn stop(
        &mut self
    ) {


        self.node =
            None;


        self.state =
            YggdrasilState::Stopped;

    }



    pub fn running(
        &self
    ) -> bool {


        matches!(
            self.state,
            YggdrasilState::Running
        )

    }

}



impl Default for YggdrasilEngine {


    fn default() -> Self {

        Self::new()

    }

}
