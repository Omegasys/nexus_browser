// Nexus IndexedDB Storage
// GPL-3.0 License


use std::collections::HashMap;


pub struct IndexedDB {


    databases:
        HashMap<String, String>,


}



impl IndexedDB {


    pub fn new() -> Self {


        Self {

            databases:
                HashMap::new(),

        }


    }



    pub fn create_database(

        &mut self,

        name: String

    ) {


        self.databases.insert(

            name.clone(),

            String::new()

        );


        println!(
            "Created IndexedDB database {}",
            name
        );


    }



    pub fn delete_database(

        &mut self,

        name: &str

    ) {


        self.databases.remove(name);


    }



    pub fn clear(

        &mut self

    ) {


        self.databases.clear();


    }


}
