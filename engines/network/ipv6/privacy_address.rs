//! IPv6 privacy address manager.
//!
//! Handles temporary IPv6 addresses.

use std::collections::HashSet;



pub struct PrivacyAddressManager {


    temporary:
        HashSet<String>,


}



impl PrivacyAddressManager {


    pub fn new() -> Self {

        Self {

            temporary:
                HashSet::new(),

        }

    }



    pub fn add(
        &mut self,
        address:String
    ) {

        self.temporary
            .insert(address);

    }



    pub fn remove(
        &mut self,
        address:&str
    ) {

        self.temporary
            .remove(address);

    }



    pub fn count(
        &self
    ) -> usize {

        self.temporary.len()

    }

}



impl Default for PrivacyAddressManager {

    fn default() -> Self {

        Self::new()

    }

}
