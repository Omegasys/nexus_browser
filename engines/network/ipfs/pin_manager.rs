//! IPFS pinning system.
//!
//! Keeps content available locally.

use std::collections::HashSet;



pub struct PinManager {


    pins:
        HashSet<String>,


}



impl PinManager {


    pub fn new() -> Self {

        Self {

            pins:
                HashSet::new(),

        }

    }



    pub fn pin(
        &mut self,
        cid:String
    ) {

        self.pins.insert(cid);

    }



    pub fn unpin(
        &mut self,
        cid:&str
    ) {

        self.pins.remove(cid);

    }



    pub fn pinned(
        &self,
        cid:&str
    ) -> bool {

        self.pins.contains(cid)

    }



    pub fn count(
        &self
    ) -> usize {

        self.pins.len()

    }

}



impl Default for PinManager {

    fn default() -> Self {

        Self::new()

    }

}
