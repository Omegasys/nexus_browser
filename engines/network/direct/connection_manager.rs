//! Direct connection tracking.

use std::collections::HashMap;



pub struct DirectConnection {


    pub destination:String,

    pub protocol:String,

    pub active:bool,


}



pub struct ConnectionManager {


    connections:
        HashMap<String,DirectConnection>,


}



impl ConnectionManager {


    pub fn new() -> Self {

        Self {

            connections:
                HashMap::new(),

        }

    }



    pub fn open(
        &mut self,
        id:String,
        destination:String,
        protocol:String
    ) {


        self.connections.insert(

            id,

            DirectConnection {

                destination,

                protocol,

                active:true,

            }

        );

    }



    pub fn close(
        &mut self,
        id:&str
    ) {

        self.connections.remove(id);

    }



    pub fn count(
        &self
    ) -> usize {

        self.connections.len()

    }

}



impl Default for ConnectionManager {

    fn default() -> Self {

        Self::new()

    }

}
