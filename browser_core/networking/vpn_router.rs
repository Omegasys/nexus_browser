// Nexus VPN Router
// GPL-3.0 License


pub struct VPNRouter {


    enabled: bool,

    provider: Option<String>,


}



impl VPNRouter {


    pub fn new() -> Self {


        Self {

            enabled: false,

            provider: None,

        }


    }



    pub fn connect(

        &mut self,

        provider:String

    ) {


        println!(
            "Connecting VPN {}",
            provider
        );


        self.provider =
            Some(provider);


        self.enabled = true;


    }



    pub fn disconnect(

        &mut self

    ) {


        println!(
            "Disconnecting VPN"
        );


        self.provider = None;

        self.enabled = false;


    }



    pub fn route(

        &self,

        destination:String

    ) {


        if self.enabled {


            println!(
                "Routing {} through VPN",
                destination
            );


        }


    }



    pub fn kill_switch_ready(

        &self

    ) -> bool {


        self.enabled


    }


}
