//! Tracker blocking engine.
//!
//! Blocks known tracking technologies,
//! advertising domains, and analytics systems.

use std::collections::HashSet;



pub struct TrackerBlocker {


    enabled: bool,

    blocked_domains: HashSet<String>,

    allowed_domains: HashSet<String>,


}



impl TrackerBlocker {


    pub fn new() -> Self {


        Self {

            enabled: true,

            blocked_domains:
                HashSet::new(),

            allowed_domains:
                HashSet::new(),

        }

    }



    pub fn enable(
        &mut self
    ) {

        self.enabled = true;

    }



    pub fn disable(
        &mut self
    ) {

        self.enabled = false;

    }



    pub fn add_blocked_domain(
        &mut self,
        domain: String
    ) {

        self.blocked_domains
            .insert(domain);

    }



    pub fn allow_domain(
        &mut self,
        domain: String
    ) {

        self.allowed_domains
            .insert(domain);

    }



    pub fn should_block(
        &self,
        domain: &str
    ) -> bool {


        if self.allowed_domains
            .contains(domain)
        {

            return false;

        }


        self.enabled &&
            self.blocked_domains
                .contains(domain)

    }



    pub fn blocked_count(
        &self
    ) -> usize {

        self.blocked_domains.len()

    }

}


impl Default for TrackerBlocker {


    fn default() -> Self {

        Self::new()

    }

}
