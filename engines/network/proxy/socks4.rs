//! SOCKS4 proxy support.

pub struct Socks4Proxy {


    enabled:bool,


}



impl Socks4Proxy {


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
