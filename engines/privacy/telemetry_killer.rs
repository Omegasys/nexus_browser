//! Telemetry blocking system.
//!
//! Prevents unwanted browser analytics
//! and diagnostic reporting.

use std::collections::HashSet;



pub struct TelemetryKiller {


    enabled: bool,

    blocked_endpoints: HashSet<String>,


}



impl TelemetryKiller {


    pub fn new() -> Self {


        Self {

            enabled: true,

            blocked_endpoints:
                HashSet::new(),

        }

    }



    pub fn block_endpoint(
        &mut self,
        endpoint: String
    ) {

        self.blocked_endpoints
            .insert(endpoint);

    }



    pub fn allow_endpoint(
        &mut self,
        endpoint: &str
    ) {

        self.blocked_endpoints
            .remove(endpoint);

    }



    pub fn should_block(
        &self,
        endpoint: &str
    ) -> bool {

        self.enabled &&
            self.blocked_endpoints
                .contains(endpoint)

    }



    pub fn disable(
        &mut self
    ) {

        self.enabled = false;

    }



    pub fn enable(
        &mut self
    ) {

        self.enabled = true;

    }

}


impl Default for TelemetryKiller {

    fn default() -> Self {

        Self::new()

    }

}
