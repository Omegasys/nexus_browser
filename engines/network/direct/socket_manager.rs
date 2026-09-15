//! Socket lifecycle management.


use std::collections::HashSet;



pub struct SocketManager {


    sockets:
        HashSet<String>,


}



impl SocketManager {


    pub fn new() -> Self {

        Self {

            sockets:
                HashSet::new(),

        }

    }



    pub fn register(
        &mut self,
        socket:String
    ) {

        self.sockets.insert(socket);

    }



    pub fn close(
        &mut self,
        socket:&str
    ) {

        self.sockets.remove(socket);

    }



    pub fn count(
        &self
    ) -> usize {

        self.sockets.len()

    }

}



impl Default for SocketManager {

    fn default() -> Self {

        Self::new()

    }

}
