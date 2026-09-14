// Nexus Network Lock
// GPL-3.0 License


pub struct NetworkLock {


    locked: bool,


    allowed_interface:
        Option<String>,


}



impl NetworkLock {


    pub fn new() -> Self {


        Self {

            locked: false,

            allowed_interface: None,

        }


    }



    pub fn lock(

        &mut self,

        interface:String

    ) {


        println!(
            "Locking network to interface {}",
            interface
        );


        self.allowed_interface =
            Some(interface);


        self.locked = true;


    }



    pub fn unlock(

        &mut self

    ) {


        println!(
            "Removing network lock"
        );


        self.locked = false;

        self.allowed_interface = None;


    }



    pub fn is_locked(

        &self

    ) -> bool {


        self.locked


    }



    pub fn interface(

        &self

    ) -> Option<&String> {


        self.allowed_interface
            .as_ref()


    }


}
