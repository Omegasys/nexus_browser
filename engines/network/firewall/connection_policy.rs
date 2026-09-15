//! Connection permission rules.


pub struct ConnectionPolicy {


    allow_connections:bool,


}



impl ConnectionPolicy {


    pub fn new() -> Self {

        Self {

            allow_connections:true,

        }

    }



    pub fn allow(
        &mut self
    ) {

        self.allow_connections =
            true;

    }



    pub fn block(
        &mut self
    ) {

        self.allow_connections =
            false;

    }



    pub fn permitted(
        &self
    ) -> bool {

        self.allow_connections

    }

}
