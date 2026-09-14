// Nexus Local Storage
// GPL-3.0 License


use std::collections::HashMap;


pub struct LocalStorage {


    data:
        HashMap<String,String>,


}



impl LocalStorage {


    pub fn new() -> Self {


        Self {

            data:
                HashMap::new(),

        }


    }



    pub fn set(

        &mut self,

        key:String,

        value:String

    ) {


        self.data.insert(
            key,
            value
        );


    }



    pub fn get(

        &self,

        key:&str

    ) -> Option<&String> {


        self.data.get(key)


    }



    pub fn clear(

        &mut self

    ) {


        self.data.clear();


    }


}
