//! Core proxy engine.

use super::proxy_manager::ProxyManager;


#[derive(Debug,Clone)]

pub enum ProxyState {

    Disabled,

    Starting,

    Active,

    Failed,

}



pub struct ProxyEngine {


    state:ProxyState,

    manager:ProxyManager,


}



impl ProxyEngine {


    pub fn new() -> Self {

        Self {

            state:
                ProxyState::Disabled,

            manager:
                ProxyManager::new(),

        }

    }



    pub fn enable(
        &mut self
    ) {

        self.state =
            ProxyState::Starting;


        self.state =
            ProxyState::Active;

    }



    pub fn disable(
        &mut self
    ) {

        self.state =
            ProxyState::Disabled;

    }



    pub fn active(
        &self
    ) -> bool {

        matches!(
            self.state,
            ProxyState::Active
        )

    }

}



impl Default for ProxyEngine {

    fn default() -> Self {

        Self::new()

    }

}
