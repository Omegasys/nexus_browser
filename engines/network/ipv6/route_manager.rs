//! IPv6 routing manager.

use std::collections::HashMap;



pub struct Ipv6Route {


    pub destination:String,

    pub gateway:String,

    pub prefix:u8,

    pub active:bool,


}



pub struct RouteManager {


    routes:
        HashMap<String,Ipv6Route>,


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
        gateway:String,
        prefix:u8
    ) {


        self.routes.insert(

            destination.clone(),

            Ipv6Route {

                destination,

                gateway,

                prefix,

                active:true,

            }

        );

    }



    pub fn remove_route(
        &mut self,
        destination:&str
    ) {


        self.routes.remove(destination);

    }



    pub fn count(
        &self
    ) -> usize {

        self.routes.len()

    }

}



impl Default for RouteManager {

    fn default() -> Self {

        Self::new()

    }

}
