//! Proxy failover manager.

pub struct ProxyFailover {


    enabled:bool,


}



impl ProxyFailover {


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
