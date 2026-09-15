//! Lokinet route manager.
//!
//! Controls LLARP routing paths.

use std::collections::HashMap;



#[derive(Debug, Clone)]

pub struct LokinetRoute {


    pub name:String,

    pub destination:String,

    pub active:bool,


}



pub struct RouteManager {


    routes:
        HashMap<String,LokinetRoute>,


}



impl RouteManager {


    pub fn new() -> Self {


        Self {

            routes:
                HashMap::new(),

        }

    }



    pub fn add_route(
        &mut self,
        name:String,
        destination:String
    ) {


        self.routes.insert(

            name.clone(),

            LokinetRoute {

                name,

                destination,

                active:
                    true,

            }

        );

    }



    pub fn remove_route(
        &mut self,
        name:&str
    ) {


        self.routes
            .remove(name);

    }



    pub fn disable_route(
        &mut self,
        name:&str
    ) {


        if let Some(
            route
        ) =
            self.routes.get_mut(name)
        {

            route.active =
                false;

        }

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



impl Default for RouteManager {


    fn default() -> Self {

        Self::new()

    }

}
