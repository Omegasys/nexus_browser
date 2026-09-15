//! Traffic rule storage.


use std::collections::HashMap;



pub struct RuleManager {


    rules:
        HashMap<String,bool>,


}



impl RuleManager {


    pub fn new() -> Self {

        Self {

            rules:
                HashMap::new(),

        }

    }



    pub fn add_rule(
        &mut self,
        name:String,
        enabled:bool
    ) {

        self.rules.insert(
            name,
            enabled
        );

    }



    pub fn enabled(
        &self,
        name:&str
    ) -> bool {

        *self.rules
            .get(name)
            .unwrap_or(&false)

    }

}



impl Default for RuleManager {

    fn default() -> Self {

        Self::new()

    }

}
