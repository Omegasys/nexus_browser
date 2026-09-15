//! Onion service manager.
//!
//! Handles local onion service configuration.

use std::collections::HashMap;



pub struct OnionService {


    pub name: String,

    pub address: Option<String>,


}



pub struct OnionServiceManager {


    services:
        HashMap<String,OnionService>,


}



impl OnionServiceManager {


    pub fn new() -> Self {


        Self {

            services:
                HashMap::new(),

        }

    }



    pub fn register(
        &mut self,
        name:String
    ) {


        self.services.insert(

            name.clone(),

            OnionService {

                name,

                address:
                    None,

            }

        );

    }



    pub fn count(
        &self
    ) -> usize {

        self.services.len()

    }

}


impl Default for OnionServiceManager {

    fn default() -> Self {

        Self::new()

    }

}
