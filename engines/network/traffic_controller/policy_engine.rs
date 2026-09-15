//! Network policy enforcement.

use super::routing_policy::RoutingPolicy;


pub struct PolicyEngine {


    routing:
        RoutingPolicy,


}



impl PolicyEngine {


    pub fn new() -> Self {

        Self {

            routing:
                RoutingPolicy::new(),

        }

    }



    pub fn routing(
        &self
    ) -> &RoutingPolicy {

        &self.routing

    }

}



impl Default for PolicyEngine {

    fn default() -> Self {

        Self::new()

    }

}
