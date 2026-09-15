//! Tor identity management.
//!
//! Controls identity separation.

use std::collections::HashMap;



pub struct TorIdentity {


    pub name:String,

    pub isolated:bool,


}



pub struct TorIdentityManager {


    identities:
        HashMap<String,TorIdentity>,


}



impl TorIdentityManager {


    pub fn new() -> Self {


        Self {

            identities:
                HashMap::new(),

        }

    }



    pub fn create_identity(
        &mut self,
        name:String
    ) {


        self.identities.insert(

            name.clone(),

            TorIdentity {

                name,

                isolated:
                    true,

            }

        );

    }



    pub fn remove_identity(
        &mut self,
        name:&str
    ) {


        self.identities.remove(name);

    }

}


impl Default for TorIdentityManager {

    fn default() -> Self {

        Self::new()

    }

}
