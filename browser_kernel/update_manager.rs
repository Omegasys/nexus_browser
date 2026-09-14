// Nexus Browser Update Manager
// GPL-3.0 License


pub struct UpdateManager {


    current_version: String,


    update_available: bool,


}



impl UpdateManager {


    pub fn new() -> Self {


        Self {

            current_version:
                "0.1.0".to_string(),

            update_available:
                false,

        }


    }



    pub fn check_updates(
        &mut self
    ) {


        println!(
            "Checking for Nexus updates"
        );


        self.update_available =
            false;


    }



    pub fn install_update(
        &self,
        version: String
    ) {


        println!(
            "Installing version {}",
            version
        );


    }



    pub fn rollback(
        &self
    ) {


        println!(
            "Rolling back update"
        );


    }



    pub fn version(
        &self
    ) -> &str {


        &self.current_version


    }


}
