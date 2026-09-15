//! Firewall integration layer.


pub struct FirewallController {


    enabled:bool,


}



impl FirewallController {


    pub fn new() -> Self {

        Self {

            enabled:false,

        }

    }



    pub fn enable(
        &mut self
    ) {

        self.enabled =
            true;

    }



    pub fn disable(
        &mut self
    ) {

        self.enabled =
            false;

    }



    pub fn active(
        &self
    ) -> bool {

        self.enabled

    }

}



impl Default for FirewallController {

    fn default() -> Self {

        Self::new()

    }

}
