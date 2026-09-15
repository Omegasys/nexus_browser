//! Domain filtering.


use std::collections::HashSet;


pub struct DomainFilter {


    blocked:
        HashSet<String>,


}



impl DomainFilter {


    pub fn new() -> Self {

        Self {

            blocked:
                HashSet::new(),

        }

    }



    pub fn block(
        &mut self,
        domain:String
    ) {

        self.blocked.insert(domain);

    }



    pub fn allowed(
        &self,
        domain:&str
    ) -> bool {

        !self.blocked.contains(domain)

    }

}
