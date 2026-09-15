//! Yggdrasil mesh routing.
//!
//! Handles peer path selection.

use std::collections::HashMap;



pub struct MeshRoute {


    pub destination:String,

    pub hops:u32,

    pub active:bool,


}



pub struct RoutingManager {


    routes:
        HashMap<String,MeshRoute>,


}



impl RoutingManager {


    pub fn new() -> Self {

        Self {

            routes:
                HashMap::new(),

        }

    }



    pub fn add_route(
        &mut self,
        destination:String,
        hops:u32
    ) {


        self.routes.insert(

            destination.clone(),

            MeshRoute {

                destination,

                hops,

                active:true,

            }

        );

    }



    pub fn route_count(
        &self
    ) -> usize {

        self.routes.len()

    }

}



impl Default for RoutingManager {

    fn default() -> Self {

        Self::new()

    }

}
