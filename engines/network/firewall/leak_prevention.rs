//! Firewall leak prevention.


pub struct FirewallLeakPrevention {


    enabled:bool,


}



impl FirewallLeakPrevention {


    pub fn new() -> Self {

        Self {

            enabled:false,

        }

    }



    pub fn enable(
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
