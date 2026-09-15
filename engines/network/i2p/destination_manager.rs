//! I2P destination identity manager.
//!
//! Handles I2P destination separation.

use std::collections::HashMap;



#[derive(Debug, Clone)]

pub struct I2pDestination {


    pub name:String,

    pub address:Option<String>,


}



pub struct DestinationManager {


    destinations:
        HashMap<String,I2pDestination>,


}



impl DestinationManager {


    pub fn new() -> Self {


        Self {

            destinations:
                HashMap::new(),

        }

    }



    pub fn create_destination(
        &mut self,
        name:String
    ) {


        self.destinations.insert(

            name.clone(),

            I2pDestination {

                name,

                address:
                    None,

            }

        );

    }



    pub fn remove_destination(
        &mut self,
        name:&str
    ) {


        self.destinations
            .remove(name);

    }



    pub fn count(
        &self
    ) -> usize {

        self.destinations.len()

    }

}



impl Default for DestinationManager {


    fn default() -> Self {

        Self::new()

    }

}
