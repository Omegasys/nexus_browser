//! HTTP proxy support.

pub struct HttpProxy {


    enabled:bool,


}



impl HttpProxy {


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
