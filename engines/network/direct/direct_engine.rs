//! Core direct networking engine.

use super::connection_manager::ConnectionManager;


#[derive(Debug,Clone)]

pub enum DirectState {

    Disabled,

    Starting,

    Active,

    Blocked,

}



pub struct DirectEngine {


    state:DirectState,

    connections:
        ConnectionManager,


}



impl DirectEngine {


    pub fn new() -> Self {

        Self {

            state:
                DirectState::Disabled,

            connections:
                ConnectionManager::new(),

        }

    }



    pub fn enable(
        &mut self
    ) {

        self.state =
            DirectState::Starting;


        self.state =
            DirectState::Active;

    }



    pub fn disable(
        &mut self
    ) {

        self.state =
            DirectState::Disabled;

    }



    pub fn block(
        &mut self
    ) {

        self.state =
            DirectState::Blocked;

    }



    pub fn active(
        &self
    ) -> bool {

        matches!(
            self.state,
            DirectState::Active
        )

    }

}



impl Default for DirectEngine {

    fn default() -> Self {

        Self::new()

    }

}
