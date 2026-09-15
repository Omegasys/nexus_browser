//! GNUnet service manager.
//!
//! Controls GNUnet subsystems.

use std::collections::HashMap;



#[derive(Debug,Clone)]

pub struct GnUnetService {


    pub name:String,

    pub running:bool,


}



pub struct ServiceManager {


    services:
        HashMap<String,GnUnetService>,


}



impl ServiceManager {


    pub fn new() -> Self {


        Self {

            services:
                HashMap::new(),

        }

    }



    pub fn enable_service(
        &mut self,
        name:String
    ) {


        self.services.insert(

            name.clone(),

            GnUnetService {

                name,

                running:
                    true,

            }

        );

    }



    pub fn disable_service(
        &mut self,
        name:&str
    ) {


        if let Some(
            service
        ) =
            self.services.get_mut(name)
        {

            service.running =
                false;

        }

    }



    pub fn service_count(
        &self
    ) -> usize {

        self.services.len()

    }

}



impl Default for ServiceManager {

    fn default() -> Self {

        Self::new()

    }

}
