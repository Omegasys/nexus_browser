//! Freenet distributed data store.
//!
//! Stores decentralized data objects.

use std::collections::HashMap;



pub struct DataStore {


    objects:
        HashMap<String,Vec<u8>>,


}



impl DataStore {


    pub fn new() -> Self {


        Self {

            objects:
                HashMap::new(),

        }

    }



    pub fn store(
        &mut self,
        key:String,
        data:Vec<u8>
    ) {


        self.objects
            .insert(
                key,
                data
            );

    }



    pub fn retrieve(
        &self,
        key:&str
    ) -> Option<&Vec<u8>> {


        self.objects.get(key)

    }



    pub fn count(
        &self
    ) -> usize {

        self.objects.len()

    }

}



impl Default for DataStore {


    fn default() -> Self {

        Self::new()

    }

}
