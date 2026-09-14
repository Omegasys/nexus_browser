// Nexus Tor Router
// GPL-3.0 License


pub struct TorRouter {


    enabled: bool,

    bridges_enabled: bool,

    onion_support: bool,


}



impl TorRouter {


    pub fn new() -> Self {


        Self {

            enabled: false,

            bridges_enabled: false,

            onion_support: true,

        }


    }



    pub fn enable(

        &mut self

    ) {


        println!(
            "Enabling Tor routing"
        );


        self.enabled = true;


    }



    pub fn disable(

        &mut self

    ) {


        self.enabled = false;


    }



    pub fn enable_bridges(

        &mut self

    ) {


        println!(
            "Tor bridge mode enabled"
        );


        self.bridges_enabled = true;


    }



    pub fn route(

        &self,

        destination:String

    ) {


        if self.enabled {


            println!(
                "Routing {} through Tor",
                destination
            );


        }


    }



    pub fn supports_onion(

        &self

    ) -> bool {


        self.onion_support


    }


}
