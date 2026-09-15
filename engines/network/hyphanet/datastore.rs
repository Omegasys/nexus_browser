//! Hyphanet distributed datastore.

use std::collections::HashMap;



pub struct DataStore {


    entries:
        HashMap<String,Vec<u8>>,


}



impl DataStore {


    pub fn new() -> Self {


        Self {

            entries:
                HashMap::new(),

        }

    }



    pub fn insert(
        &mut self,
        key:String,
        data:Vec<u8>
    ) {


        self.entries.insert(
            key,
            data
        );

    }



    pub fn get(
        &self,
        key:&str
    ) -> Option<&Vec<u8>> {

        self.entries.get(key)

    }



    pub fn count(
        &self
    ) -> usize {

        self.entries.len()

    }

}



impl Default for DataStore {

    fn default() -> Self {

        Self::new()

    }

}
