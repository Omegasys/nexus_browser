//! Network interface selection.


use std::collections::HashMap;



pub struct Interface {


    pub name:String,

    pub enabled:bool,


}



pub struct InterfaceSelector {


    interfaces:
        HashMap<String,Interface>,


}



impl InterfaceSelector {


    pub fn new() -> Self {

        Self {

            interfaces:
                HashMap::new(),

        }

    }



    pub fn add(
        &mut self,
        name:String
    ) {


        self.interfaces.insert(

            name.clone(),

            Interface {

                name,

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

}



impl Default for InterfaceSelector {

    fn default() -> Self {

        Self::new()

    }

}
