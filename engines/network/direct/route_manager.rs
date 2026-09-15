//! Direct routing manager.


use std::collections::HashMap;



pub struct DirectRoute {


    pub destination:String,

    pub gateway:String,


}



pub struct RouteManager {


    routes:
        HashMap<String,DirectRoute>,


}



impl RouteManager {


    pub fn new() -> Self {

        Self {

            routes:
                HashMap::new(),

        }

    }



    pub fn add(
        &mut self,
        destination:String,
        gateway:String
    ) {


        self.routes.insert(

            destination.clone(),

            DirectRoute {

                destination,

                gateway,

            }

        );

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
