//! HTTPS CONNECT proxy support.

pub struct HttpsProxy {


    enabled:bool,


}



impl HttpsProxy {


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
