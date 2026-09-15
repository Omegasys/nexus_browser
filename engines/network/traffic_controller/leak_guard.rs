//! Prevents traffic escaping assigned routes.


pub struct LeakGuard {


    enabled:bool,


}



impl LeakGuard {


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



impl Default for LeakGuard {

    fn default() -> Self {

        Self::new()

    }

}
