//! I2P tunnel management.
//!
//! Controls inbound and outbound tunnels.

use std::collections::HashMap;



#[derive(Debug, Clone)]

pub enum TunnelType {

    Inbound,

    Outbound,

}



#[derive(Debug, Clone)]

pub struct I2pTunnel {


    pub id: String,

    pub tunnel_type: TunnelType,

    pub active: bool,


}



pub struct TunnelManager {


    tunnels:
        HashMap<String,I2pTunnel>,


}



impl TunnelManager {


    pub fn new() -> Self {


        Self {

            tunnels:
                HashMap::new(),

        }

    }



    pub fn create_tunnel(
        &mut self,
        id:String,
        tunnel_type:TunnelType
    ) {


        self.tunnels.insert(

            id.clone(),

            I2pTunnel {

                id,

                tunnel_type,

                active:
                    true,

            }

        );

    }



    pub fn close_tunnel(
        &mut self,
        id:&str
    ) {


        if let Some(
            tunnel
        ) =
            self.tunnels.get_mut(id)
        {

            tunnel.active =
                false;

        }

    }



    pub fn active_count(
        &self
    ) -> usize {


        self.tunnels
            .values()
            .filter(
                |t|
                t.active
            )
            .count()

    }

}



impl Default for TunnelManager {


    fn default() -> Self {

        Self::new()

    }

}
