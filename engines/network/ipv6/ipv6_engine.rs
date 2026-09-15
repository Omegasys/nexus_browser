//! Core IPv6 engine.

use super::interface_manager::InterfaceManager;


#[derive(Debug, Clone)]

pub enum Ipv6State {

    Disabled,

    Initializing,

    Active,

    Blocked,

}



pub struct Ipv6Engine {


    state:Ipv6State,

    interfaces:
        Option<InterfaceManager>,


}



impl Ipv6Engine {


    pub fn new() -> Self {

        Self {

            state:
                Ipv6State::Disabled,

            interfaces:
                None,

        }

    }



    pub fn enable(
        &mut self
    ) {


        self.state =
            Ipv6State::Initializing;


        self.interfaces =
            Some(
                InterfaceManager::new()
            );


        self.state =
            Ipv6State::Active;

    }



    pub fn disable(
        &mut self
    ) {


        self.interfaces =
            None;


        self.state =
            Ipv6State::Disabled;

    }



    pub fn block(
        &mut self
    ) {

        self.state =
            Ipv6State::Blocked;

    }



    pub fn active(
        &self
    ) -> bool {


        matches!(
            self.state,
            Ipv6State::Active
        )

    }

}



impl Default for Ipv6Engine {

    fn default() -> Self {

        Self::new()

    }

}
