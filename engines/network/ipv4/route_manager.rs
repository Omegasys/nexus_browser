//! IPv4 route management.

use std::collections::HashMap;



pub struct Ipv4Route {


    pub destination:String,

    pub gateway:String,

    pub active:bool,


}



pub struct RouteManager {


    routes:
        HashMap<String,Ipv4Route>,


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
        destination:String,
        gateway:String
    ) {


        self.routes.insert(

            destination.clone(),

            Ipv4Route {

                destination,

                gateway,

                active:true,

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



impl Default for RouteManager {


    fn default() -> Self {

        Self::new()

    }

}
