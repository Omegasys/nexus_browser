//! Protocol filtering and control.


use std::collections::HashSet;



pub struct ProtocolManager {


    allowed:
        HashSet<String>,


}



impl ProtocolManager {


    pub fn new() -> Self {

        Self {

            allowed:
                HashSet::new(),

        }

    }



    pub fn allow(
        &mut self,
        protocol:String
    ) {

        self.allowed.insert(protocol);

    }



    pub fn blocked(
        &self,
        protocol:&str
    ) -> bool {

        !self.allowed.contains(protocol)

    }

}



impl Default for ProtocolManager {

    fn default() -> Self {

        Self::new()

    }

}
