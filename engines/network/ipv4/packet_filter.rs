//! IPv4 packet filtering.

use std::collections::HashSet;



pub struct PacketFilter {


    blocked_addresses:
        HashSet<String>,


    enabled:
        bool,


}



impl PacketFilter {


    pub fn new() -> Self {


        Self {

            blocked_addresses:
                HashSet::new(),

            enabled:
                false,

        }

    }



    pub fn enable(
        &mut self
    ) {

        self.enabled =
            true;

    }



    pub fn block_address(
        &mut self,
        address:String
    ) {


        self.blocked_addresses
            .insert(address);

    }



    pub fn allowed(
        &self,
        address:&str
    ) -> bool {


        if !self.enabled {

            return true;

        }


        !self.blocked_addresses
            .contains(address)

    }

}



impl Default for PacketFilter {

    fn default() -> Self {

        Self::new()

    }

}
