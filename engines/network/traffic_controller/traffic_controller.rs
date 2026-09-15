//! Main traffic controller.

use super::policy_engine::PolicyEngine;


pub struct TrafficController {


    enabled:bool,

    policy:
        PolicyEngine,


}



impl TrafficController {


    pub fn new() -> Self {

        Self {

            enabled:false,

            policy:
                PolicyEngine::new(),

        }

    }



    pub fn enable(
        &mut self
    ) {

        self.enabled =
            true;

    }



    pub fn disable(
        &mut self
    ) {

        self.enabled =
            false;

    }



    pub fn active(
        &self
    ) -> bool {

        self.enabled

    }



    pub fn policy(
        &self
    ) -> &PolicyEngine {

        &self.policy

    }

}



impl Default for TrafficController {

    fn default() -> Self {

        Self::new()

    }

}
