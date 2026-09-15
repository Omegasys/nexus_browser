//! Freenet identity management.
//!
//! Controls publishing identities.

use std::collections::HashMap;



pub struct FreenetIdentity {


    pub name:String,

    pub public_key:String,


}



pub struct FreenetIdentityManager {


    identities:
        HashMap<String,FreenetIdentity>,


}



impl FreenetIdentityManager {


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

            FreenetIdentity {

                name,

                public_key:
                    String::new(),

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



impl Default for FreenetIdentityManager {


    fn default() -> Self {

        Self::new()

    }

}
