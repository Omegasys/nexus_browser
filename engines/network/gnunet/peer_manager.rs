//! GNUnet peer management.
//!
//! Tracks decentralized peers.

use std::collections::HashMap;



#[derive(Debug,Clone)]

pub struct GnUnetPeer {


    pub identity:String,

    pub address:String,

    pub connected:bool,


}



pub struct PeerManager {


    peers:
        HashMap<String,GnUnetPeer>,


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
        identity:String,
        address:String
    ) {


        self.peers.insert(

            identity.clone(),

            GnUnetPeer {

                identity,

                address,

                connected:
                    true,

            }

        );

    }



    pub fn remove_peer(
        &mut self,
        identity:&str
    ) {


        self.peers.remove(identity);

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
