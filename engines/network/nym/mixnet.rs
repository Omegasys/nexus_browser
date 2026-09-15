//! Nym mixnet routing layer.
//!
//! Controls anonymous packet routing.

use std::collections::VecDeque;



#[derive(Debug, Clone)]

pub struct MixNode {


    pub identity: String,

    pub active: bool,


}



pub struct Mixnet {


    nodes:
        VecDeque<MixNode>,


    connected:
        bool,


}



impl Mixnet {


    pub fn new() -> Self {


        Self {

            nodes:
                VecDeque::new(),

            connected:
                false,

        }

    }



    pub fn add_node(
        &mut self,
        identity:String
    ) {


        self.nodes
            .push_back(

                MixNode {

                    identity,

                    active:
                        true,

                }

            );

    }



    pub fn connect(
        &mut self
    ) {


        self.connected =
            true;

    }



    pub fn disconnect(
        &mut self
    ) {


        self.connected =
            false;

    }



    pub fn connected(
        &self
    ) -> bool {

        self.connected

    }



    pub fn node_count(
        &self
    ) -> usize {

        self.nodes.len()

    }

}



impl Default for Mixnet {


    fn default() -> Self {

        Self::new()

    }

}
