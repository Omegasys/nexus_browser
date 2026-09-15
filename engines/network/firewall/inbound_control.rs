//! Incoming traffic control.


pub struct InboundControl {


    enabled:bool,


}



impl InboundControl {


    pub fn new() -> Self {

        Self {

            enabled:false,

        }

    }



    pub fn enabled(
        &self
    ) -> bool {

        self.enabled

    }

}
