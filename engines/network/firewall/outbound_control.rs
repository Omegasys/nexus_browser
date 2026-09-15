//! Outbound traffic control.


pub struct OutboundControl {


    enabled:bool,


}



impl OutboundControl {


    pub fn new() -> Self {

        Self {

            enabled:true,

        }

    }



    pub fn enabled(
        &self
    ) -> bool {

        self.enabled

    }

}
