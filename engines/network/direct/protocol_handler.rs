//! Handles allowed direct protocols.


use std::collections::HashSet;



pub struct ProtocolHandler {


    protocols:
        HashSet<String>,


}



impl ProtocolHandler {


    pub fn new() -> Self {

        Self {

            protocols:
                HashSet::new(),

        }

    }



    pub fn allow(
        &mut self,
        protocol:String
    ) {

        self.protocols.insert(protocol);

    }



    pub fn allowed(
        &self,
        protocol:&str
    ) -> bool {

        self.protocols.contains(protocol)

    }

}



impl Default for ProtocolHandler {

    fn default() -> Self {

        Self::new()

    }

}
