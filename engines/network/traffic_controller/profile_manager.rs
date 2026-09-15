//! Traffic profile management.


use std::collections::HashMap;



pub struct TrafficProfile {


    pub name:String,

    pub description:String,


}



pub struct TrafficProfileManager {


    profiles:
        HashMap<String,TrafficProfile>,


}



impl TrafficProfileManager {


    pub fn new() -> Self {

        Self {

            profiles:
                HashMap::new(),

        }

    }



    pub fn create(
        &mut self,
        name:String,
        description:String
    ) {


        self.profiles.insert(

            name.clone(),

            TrafficProfile {

                name,

                description,

            }

        );

    }

}



impl Default for TrafficProfileManager {

    fn default() -> Self {

        Self::new()

    }

}
