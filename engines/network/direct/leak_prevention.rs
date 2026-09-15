//! Direct connection leak prevention.
//!
//! Used when traffic must stay inside
//! another routing layer.


pub struct DirectLeakPrevention {


    enabled:bool,


}



impl DirectLeakPrevention {


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



impl Default for DirectLeakPrevention {

    fn default() -> Self {

        Self::new()

    }

}
