//! Port filtering.


use std::collections::HashSet;


pub struct PortFilter {


    blocked:
        HashSet<u16>,


}



impl PortFilter {


    pub fn new() -> Self {

        Self {

            blocked:
                HashSet::new(),

        }

    }



    pub fn block(
        &mut self,
        port:u16
    ) {

        self.blocked.insert(port);

    }



    pub fn allowed(
        &self,
        port:u16
    ) -> bool {

        !self.blocked.contains(&port)

    }

}
