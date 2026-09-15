//! GNUnet identity manager.
//!
//! Separates user identities
//! across Nexus profiles.

use std::collections::HashMap;



pub struct GnUnetIdentity {


    pub name:String,

    pub public_key:String,


}



pub struct GnUnetIdentityManager {


    identities:
        HashMap<String,GnUnetIdentity>,


}



impl GnUnetIdentityManager {


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

            GnUnetIdentity {

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



impl Default for GnUnetIdentityManager {

    fn default() -> Self {

        Self::new()

    }

}
