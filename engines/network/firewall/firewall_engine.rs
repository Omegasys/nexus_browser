//! Main firewall controller.

use super::rule_engine::RuleEngine;


#[derive(Debug,Clone)]

pub enum FirewallState {

    Disabled,

    Active,

    Locked,

}



pub struct FirewallEngine {


    state:FirewallState,

    rules:RuleEngine,


}



impl FirewallEngine {


    pub fn new() -> Self {

        Self {

            state:
                FirewallState::Disabled,

            rules:
                RuleEngine::new(),

        }

    }



    pub fn enable(
        &mut self
    ) {

        self.state =
            FirewallState::Active;

    }



    pub fn disable(
        &mut self
    ) {

        self.state =
            FirewallState::Disabled;

    }



    pub fn lock(
        &mut self
    ) {

        self.state =
            FirewallState::Locked;

    }



    pub fn active(
        &self
    ) -> bool {

        matches!(
            self.state,
            FirewallState::Active
        )

    }

}



impl Default for FirewallEngine {

    fn default() -> Self {

        Self::new()

    }

}
