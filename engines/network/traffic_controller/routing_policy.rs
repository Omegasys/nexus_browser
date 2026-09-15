//! Defines traffic routing decisions.


#[derive(Debug,Clone)]

pub enum RouteTarget {


    Direct,

    VPN,

    Tor,

    I2P,

    Nym,

    Lokinet,

    Yggdrasil,

    IPFS,

    Freenet,

    GNUnet,

    Hyphanet,


}



pub struct RoutingPolicy {


    target:
        RouteTarget,


}



impl RoutingPolicy {


    pub fn new() -> Self {

        Self {

            target:
                RouteTarget::Direct,

        }

    }



    pub fn set_target(
        &mut self,
        target:RouteTarget
    ) {

        self.target =
            target;

    }



    pub fn target(
        &self
    ) -> &RouteTarget {

        &self.target

    }

}



impl Default for RoutingPolicy {

    fn default() -> Self {

        Self::new()

    }

}
