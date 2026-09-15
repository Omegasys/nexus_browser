//! IPFS content addressing.
//!
//! Manages content identifiers (CID).

use std::collections::HashMap;



pub struct ContentManager {


    content:
        HashMap<String,String>,


}



impl ContentManager {


    pub fn new() -> Self {

        Self {

            content:
                HashMap::new(),

        }

    }



    pub fn register_content(
        &mut self,
        cid:String,
        location:String
    ) {


        self.content.insert(

            cid,

            location

        );

    }



    pub fn get_content(
        &self,
        cid:&str
    ) -> Option<&String> {

        self.content.get(cid)

    }



    pub fn count(
        &self
    ) -> usize {

        self.content.len()

    }

}



impl Default for ContentManager {

    fn default() -> Self {

        Self::new()

    }

}
