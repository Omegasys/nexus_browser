//! GNUnet decentralized routing.
//!
//! Handles peer-to-peer paths.

use std::collections::HashMap;



#[derive(Debug,Clone)]

pub struct GnUnetRoute {


    pub destination:String,

    pub hops:u32,

    pub active:bool,


}



pub struct RoutingManager {


    routes:
        HashMap<String,GnUnetRoute>,


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

            GnUnetRoute {

                destination,

                hops,

                active:
                    true,

            }

        );

    }



    pub fn remove_route(
        &mut self,
        destination:&str
    ) {


        self.routes
            .remove(destination);

    }



    pub fn active_routes(
        &self
    ) -> usize {


        self.routes
            .values()
            .filter(
                |r|
                r.active
            )
            .count()

    }

}



impl Default for RoutingManager {

    fn default() -> Self {

        Self::new()

    }

}
