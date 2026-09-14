// Nexus Session Storage
// GPL-3.0 License


use std::collections::HashMap;


pub struct SessionStorage {


    values:
        HashMap<String,String>,


}



impl SessionStorage {


    pub fn new() -> Self {


        Self {

            values:
                HashMap::new(),

        }


    }



    pub fn insert(

        &mut self,

        key:String,

        value:String

    ) {


        self.values.insert(
            key,
            value
        );


    }



    pub fn remove(

        &mut self,

        key:&str

    ) {


        self.values.remove(key);


    }



    pub fn clear(

        &mut self

    ) {


        self.values.clear();


    }


}
