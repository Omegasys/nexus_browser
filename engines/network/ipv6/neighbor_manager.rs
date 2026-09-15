//! IPv6 neighbor discovery manager.

use std::collections::HashMap;



pub struct Neighbor {


    pub address:String,

    pub interface:String,

    pub reachable:bool,


}



pub struct NeighborManager {


    neighbors:
        HashMap<String,Neighbor>,


}



impl NeighborManager {


    pub fn new() -> Self {

        Self {

            neighbors:
                HashMap::new(),

        }

    }



    pub fn add(
        &mut self,
        address:String,
        interface:String
    ) {


        self.neighbors.insert(

            address.clone(),

            Neighbor {

                address,

                interface,

                reachable:true,

            }

        );

    }



    pub fn remove(
        &mut self,
        address:&str
    ) {

        self.neighbors.remove(address);

    }



    pub fn count(
        &self
    ) -> usize {

        self.neighbors.len()

    }

}



impl Default for NeighborManager {

    fn default() -> Self {

        Self::new()

    }

}
