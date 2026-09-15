//! Firewall rule storage.


#[derive(Debug,Clone)]

pub enum RuleAction {

    Allow,

    Block,

}



pub struct FirewallRule {


    pub name:String,

    pub action:RuleAction,


}



pub struct RuleManager {


    rules:
        Vec<FirewallRule>,


}



impl RuleManager {


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

}



impl Default for RuleManager {

    fn default() -> Self {

        Self::new()

    }

}
