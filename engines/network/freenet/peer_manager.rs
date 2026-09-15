//! Freenet peer management.
//!
//! Handles decentralized peer connections.

use std::collections::HashMap;



#[derive(Debug, Clone)]

pub struct FreenetPeer {


    pub id:String,

    pub address:String,

    pub trusted:bool,


}



pub struct PeerManager {


    peers:
        HashMap<String,FreenetPeer>,


}



impl PeerManager {


    pub fn new() -> Self {


        Self {

            peers:
                HashMap::new(),

        }

    }



    pub fn add_peer(
        &mut self,
        id:String,
        address:String
    ) {


        self.peers.insert(

            id.clone(),

            FreenetPeer {

                id,

                address,

                trusted:
                    false,

            }

        );

    }



    pub fn remove_peer(
        &mut self,
        id:&str
    ) {


        self.peers.remove(id);

    }



    pub fn count(
        &self
    ) -> usize {

        self.peers.len()

    }

}



impl Default for PeerManager {

    fn default() -> Self {

        Self::new()

    }

}
