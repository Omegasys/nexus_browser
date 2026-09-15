//! Firewall emergency lock.


pub struct FirewallKillSwitch {


    enabled:bool,


}



impl FirewallKillSwitch {


    pub fn new() -> Self {

        Self {

            enabled:false,

        }

    }



    pub fn activate(
        &mut self
    ) {

        self.enabled=true;

    }



    pub fn active(
        &self
    ) -> bool {

        self.enabled

    }

}
