//! Protocol filtering.


use std::collections::HashSet;


pub struct ProtocolFilter {


    blocked:
        HashSet<String>,


}



impl ProtocolFilter {


    pub fn new() -> Self {

        Self {

            blocked:
                HashSet::new(),

        }

    }



    pub fn block(
        &mut self,
        protocol:String
    ) {

        self.blocked.insert(protocol);

    }



    pub fn allowed(
        &self,
        protocol:&str
    ) -> bool {

        !self.blocked.contains(protocol)

    }

}
