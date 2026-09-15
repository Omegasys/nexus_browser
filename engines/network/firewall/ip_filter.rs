//! IP address filtering.


use std::collections::HashSet;


pub struct IpFilter {


    blocked:
        HashSet<String>,


}



impl IpFilter {


    pub fn new() -> Self {

        Self {

            blocked:
                HashSet::new(),

        }

    }



    pub fn block(
        &mut self,
        ip:String
    ) {

        self.blocked.insert(ip);

    }



    pub fn allowed(
        &self,
        ip:&str
    ) -> bool {

        !self.blocked.contains(ip)

    }

}
