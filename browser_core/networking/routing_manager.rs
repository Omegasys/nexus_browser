// Nexus Network Routing Manager
// GPL-3.0 License


#[derive(Debug, Clone)]
pub enum NetworkRoute {


    Direct,

    VPN(String),

    Tor,

    I2P,

    Lokinet,

    Nym,

}



pub struct RoutingManager {


    active_routes:
        Vec<NetworkRoute>,


}



impl RoutingManager {


    pub fn new() -> Self {


        Self {

            active_routes:
                Vec::new(),

        }


    }



    pub fn add_route(

        &mut self,

        route:NetworkRoute

    ) {


        println!(
            "Adding network route {:?}",
            route
        );


        self.active_routes.push(
            route
        );


    }



    pub fn remove_route(

        &mut self

    ) {


        self.active_routes.clear();


    }



    pub fn route_for_tab(

        &self,

        tab_id:u64

    ) -> Option<&NetworkRoute> {


        println!(
            "Finding route for tab {}",
            tab_id
        );


        self.active_routes.first()


    }



    pub fn force_tor(

        &mut self

    ) {


        self.active_routes.clear();


        self.active_routes.push(
            NetworkRoute::Tor
        );


    }



    pub fn force_vpn(

        &mut self,

        name:String

    ) {


        self.active_routes.clear();


        self.active_routes.push(
            NetworkRoute::VPN(name)
        );


    }


}
