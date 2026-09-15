//! Core IPv4 engine.
//!
//! Controls IPv4 networking lifecycle.

use super::interface_manager::InterfaceManager;


#[derive(Debug, Clone)]

pub enum Ipv4State {

    Disabled,

    Initializing,

    Active,

    Blocked,

}



pub struct Ipv4Engine {


    state: Ipv4State,

    interfaces:
        Option<InterfaceManager>,


}



impl Ipv4Engine {


    pub fn new() -> Self {


        Self {

            state:
                Ipv4State::Disabled,

            interfaces:
                None,

        }

    }



    pub fn enable(
        &mut self
    ) {


        self.state =
            Ipv4State::Initializing;


        self.interfaces =
            Some(
                InterfaceManager::new()
            );


        self.state =
            Ipv4State::Active;

    }



    pub fn disable(
        &mut self
    ) {


        self.interfaces =
            None;


        self.state =
            Ipv4State::Disabled;

    }



    pub fn block(
        &mut self
    ) {

        self.state =
            Ipv4State::Blocked;

    }



    pub fn active(
        &self
    ) -> bool {


        matches!(
            self.state,
            Ipv4State::Active
        )

    }

}



impl Default for Ipv4Engine {


    fn default() -> Self {

        Self::new()

    }

}
