//! Hyphanet identity isolation.

use std::collections::HashMap;



pub struct HyphanetIdentity {


    pub name:String,

    pub public_key:String,


}



pub struct HyphanetIdentityManager {


    identities:
        HashMap<String,HyphanetIdentity>,


}



impl HyphanetIdentityManager {


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

            HyphanetIdentity {

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



impl Default for HyphanetIdentityManager {

    fn default() -> Self {

        Self::new()

    }

}
