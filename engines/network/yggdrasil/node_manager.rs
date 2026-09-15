//! Yggdrasil node lifecycle manager.

pub struct YggdrasilNodeManager {


    running:bool,

    node_id:String,


}



impl YggdrasilNodeManager {


    pub fn new() -> Self {


        Self {

            running:false,

            node_id:
                String::new(),

        }

    }



    pub fn start(
        &mut self
    ) {

        self.running =
            true;

    }



    pub fn stop(
        &mut self
    ) {

        self.running =
            false;

    }



    pub fn running(
        &self
    ) -> bool {

        self.running

    }



    pub fn node_id(
        &self
    ) -> &str {

        &self.node_id

    }

}



impl Default for YggdrasilNodeManager {

    fn default() -> Self {

        Self::new()

    }

}
