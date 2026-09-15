//! IPFS peer discovery and management.

use std::collections::HashMap;



#[derive(Debug, Clone)]

pub struct IpfsPeer {


    pub id:String,

    pub address:String,

    pub connected:bool,


}



pub struct PeerManager {


    peers:
        HashMap<String,IpfsPeer>,


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

            IpfsPeer {

                id,

                address,

                connected:
                    true,

            }

        );

    }



    pub fn remove_peer(
        &mut self,
        id:&str
    ) {

        self.peers.remove(id);

    }



    pub fn peer_count(
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
