//! Proxy traffic leak prevention.

pub struct ProxyLeakPrevention {


    enabled:bool,


}



impl ProxyLeakPrevention {


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
