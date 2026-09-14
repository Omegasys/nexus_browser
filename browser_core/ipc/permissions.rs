// Nexus IPC Permission System
// GPL-3.0 License


pub struct IPCPermissions {



}



impl IPCPermissions {


    pub fn new() -> Self {


        Self {}


    }



    pub fn check(
        &self,
        component: &str,
        permission: &str
    ) -> bool {


        println!(
            "Checking {} permission {}",
            component,
            permission
        );


        true


    }


}
