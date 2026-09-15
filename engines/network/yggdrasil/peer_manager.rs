//! Yggdrasil peer management.
//!
//! Tracks encrypted mesh peers.

use std::collections::HashMap;



#[derive(Debug,Clone)]

pub struct YggdrasilPeer {


    pub public_key:String,

    pub address:String,

    pub connected:bool,


}



pub struct PeerManager {


    peers:
        HashMap<String,YggdrasilPeer>,


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
        key:String,
        address:String
    ) {


        self.peers.insert(

            key.clone(),

            YggdrasilPeer {

                public_key:key,

                address,

                connected:true,

            }

        );

    }



    pub fn remove_peer(
        &mut self,
        key:&str
    ) {

        self.peers.remove(key);

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
