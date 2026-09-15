//! IPv6 packet filtering.

use std::collections::HashSet;



pub struct PacketFilter {


    blocked:
        HashSet<String>,


    enabled:
        bool,


}



impl PacketFilter {


    pub fn new() -> Self {

        Self {

            blocked:
                HashSet::new(),

            enabled:false,

        }

    }



    pub fn enable(
        &mut self
    ) {

        self.enabled =
            true;

    }



    pub fn block(
        &mut self,
        address:String
    ) {

        self.blocked
            .insert(address);

    }



    pub fn allowed(
        &self,
        address:&str
    ) -> bool {


        if !self.enabled {

            return true;

        }


        !self.blocked
            .contains(address)

    }

}



impl Default for PacketFilter {

    fn default() -> Self {

        Self::new()

    }

}
