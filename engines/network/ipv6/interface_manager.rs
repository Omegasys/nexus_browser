//! IPv6 interface manager.

use std::collections::HashMap;



#[derive(Debug,Clone)]

pub struct IPv6Interface {


    pub name:String,

    pub address:String,

    pub enabled:bool,


}



pub struct InterfaceManager {


    interfaces:
        HashMap<String,IPv6Interface>,


}



impl InterfaceManager {


    pub fn new() -> Self {

        Self {

            interfaces:
                HashMap::new(),

        }

    }



    pub fn register(
        &mut self,
        name:String,
        address:String
    ) {


        self.interfaces.insert(

            name.clone(),

            IPv6Interface {

                name,

                address,

                enabled:true,

            }

        );

    }



    pub fn disable(
        &mut self,
        name:&str
    ) {


        if let Some(interface) =
            self.interfaces.get_mut(name)
        {

            interface.enabled =
                false;

        }

    }



    pub fn count(
        &self
    ) -> usize {

        self.interfaces.len()

    }

}



impl Default for InterfaceManager {

    fn default() -> Self {

        Self::new()

    }

}
