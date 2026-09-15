//! IPv4 connection tracking.

use std::collections::HashSet;



pub struct ConnectionTracker {


    connections:
        HashSet<String>,


}



impl ConnectionTracker {


    pub fn new() -> Self {


        Self {

            connections:
                HashSet::new(),

        }

    }



    pub fn add(
        &mut self,
        connection:String
    ) {

        self.connections
            .insert(connection);

    }



    pub fn remove(
        &mut self,
        connection:&str
    ) {

        self.connections
            .remove(connection);

    }



    pub fn active(
        &self
    ) -> usize {

        self.connections.len()

    }

}



impl Default for ConnectionTracker {

    fn default() -> Self {

        Self::new()

    }

}
