//! Yggdrasil identity manager.
//!
//! Controls node identity separation.

use std::collections::HashMap;



pub struct YggdrasilIdentity {


    pub name:String,

    pub public_key:String,


}



pub struct YggdrasilIdentityManager {


    identities:
        HashMap<String,YggdrasilIdentity>,


}



impl YggdrasilIdentityManager {


    pub fn new() -> Self {

        Self {

            identities:
                HashMap::new(),

        }

    }



    pub fn create(
        &mut self,
        name:String
    ) {


        self.identities.insert(

            name.clone(),

            YggdrasilIdentity {

                name,

                public_key:
                    String::new(),

            }

        );

    }



    pub fn remove(
        &mut self,
        name:&str
    ) {

        self.identities.remove(name);

    }

}



impl Default for YggdrasilIdentityManager {

    fn default() -> Self {

        Self::new()

    }

}
