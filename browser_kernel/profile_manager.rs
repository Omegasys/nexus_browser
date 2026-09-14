// Nexus Browser Profile Manager
// GPL-3.0 License

use std::collections::HashMap;


#[derive(Clone)]
pub struct BrowserProfile {


    pub name: String,

    pub private_mode: bool,

    pub isolated: bool,

}



pub struct ProfileManager {


    profiles:
        HashMap<String, BrowserProfile>,


    active:
        Option<String>,


}



impl ProfileManager {


    pub fn new() -> Self {


        Self {

            profiles:
                HashMap::new(),

            active:
                None,

        }


    }



    pub fn create_profile(
        &mut self,
        profile: BrowserProfile
    ) {


        println!(
            "Creating profile {}",
            profile.name
        );


        self.profiles.insert(
            profile.name.clone(),
            profile
        );


    }



    pub fn activate(
        &mut self,
        name: String
    ) {


        if self.profiles.contains_key(
            &name
        ) {


            self.active =
                Some(name);


        }


    }



    pub fn delete_profile(
        &mut self,
        name: &str
    ) {


        self.profiles.remove(
            name
        );


    }



    pub fn active_profile(
        &self
    ) -> Option<String> {


        self.active.clone()


    }


}
