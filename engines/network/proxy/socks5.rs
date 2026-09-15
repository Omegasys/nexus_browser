//! SOCKS5 proxy support.

pub struct Socks5Proxy {


    enabled:bool,


}



impl Socks5Proxy {


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
