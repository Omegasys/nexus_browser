//! Hyphanet request routing.

use std::collections::HashMap;



pub struct HyphanetRoute {


    pub destination:String,

    pub hops:u32,

    pub active:bool,


}



pub struct RoutingManager {


    routes:
        HashMap<String,HyphanetRoute>,


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

            HyphanetRoute {

                destination,

                hops,

                active:
                    true,

            }

        );

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
