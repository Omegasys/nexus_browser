//! Selects network engine.


use super::routing_policy::RouteTarget;


pub struct EngineSelector;



impl EngineSelector {


    pub fn select(
        target:&RouteTarget
    ) -> String {


        match target {


            RouteTarget::VPN =>
                "vpn".into(),

            RouteTarget::Tor =>
                "tor".into(),

            RouteTarget::I2P =>
                "i2p".into(),

            RouteTarget::Nym =>
                "nym".into(),

            RouteTarget::Lokinet =>
                "lokinet".into(),

            RouteTarget::Yggdrasil =>
                "yggdrasil".into(),

            RouteTarget::IPFS =>
                "ipfs".into(),

            RouteTarget::Freenet =>
                "freenet".into(),

            RouteTarget::GNUnet =>
                "gnunet".into(),

            RouteTarget::Hyphanet =>
                "hyphanet".into(),

            _ =>
                "direct".into(),

        }

    }

}
