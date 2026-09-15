//! Firewall decision engine.

use super::rule_manager::FirewallRule;


pub struct RuleEngine {


    rules:
        Vec<FirewallRule>,


}



impl RuleEngine {


    pub fn new() -> Self {

        Self {

            rules:
                Vec::new(),

        }

    }



    pub fn add(
        &mut self,
        rule:FirewallRule
    ) {

        self.rules.push(rule);

    }



    pub fn count(
        &self
    ) -> usize {

        self.rules.len()

    }

}



impl Default for RuleEngine {

    fn default() -> Self {

        Self::new()

    }

}
