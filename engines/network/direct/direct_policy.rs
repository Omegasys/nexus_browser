//! Direct networking policy rules.


pub struct DirectPolicy {


    allow_http:bool,

    allow_https:bool,

    allow_udp:bool,


}



impl DirectPolicy {


    pub fn new() -> Self {

        Self {

            allow_http:false,

            allow_https:true,

            allow_udp:true,

        }

    }



    pub fn https_allowed(
        &self
    ) -> bool {

        self.allow_https

    }



    pub fn udp_allowed(
        &self
    ) -> bool {

        self.allow_udp

    }

}



impl Default for DirectPolicy {

    fn default() -> Self {

        Self::new()

    }

}
