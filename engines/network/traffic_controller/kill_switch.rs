//! Global network kill switch.


pub struct KillSwitch {


    enabled:bool,

    network_allowed:bool,


}



impl KillSwitch {


    pub fn new() -> Self {

        Self {

            enabled:false,

            network_allowed:true,

        }

    }



    pub fn activate(
        &mut self
    ) {

        self.enabled =
            true;


        self.network_allowed =
            false;

    }



    pub fn deactivate(
        &mut self
    ) {

        self.enabled =
            false;


        self.network_allowed =
            true;

    }



    pub fn allow_network(
        &self
    ) -> bool {

        self.network_allowed

    }

}



impl Default for KillSwitch {

    fn default() -> Self {

        Self::new()

    }

}
