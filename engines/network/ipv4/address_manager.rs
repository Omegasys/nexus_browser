//! IPv4 address management.

use std::collections::HashSet;



pub struct Ipv4AddressManager {


    addresses:
        HashSet<String>,


}



impl Ipv4AddressManager {


    pub fn new() -> Self {


        Self {

            addresses:
                HashSet::new(),

        }

    }



    pub fn add(
        &mut self,
        address:String
    ) {


        self.addresses
            .insert(address);

    }



    pub fn remove(
        &mut self,
        address:&str
    ) {


        self.addresses
            .remove(address);

    }



    pub fn count(
        &self
    ) -> usize {

        self.addresses.len()

    }

}



impl Default for Ipv4AddressManager {


    fn default() -> Self {

        Self::new()

    }

}
