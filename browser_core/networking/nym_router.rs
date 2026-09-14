// Nexus Nym Mixnet Router
// GPL-3.0 License


pub struct NymRouter {


    enabled: bool,

    mixnet_mode: bool,


}



impl NymRouter {


    pub fn new() -> Self {


        Self {

            enabled: false,

            mixnet_mode: true,

        }


    }



    pub fn enable(

        &mut self

    ) {


        println!(
            "Enabling Nym mixnet routing"
        );


        self.enabled = true;


    }



    pub fn disable(

        &mut self

    ) {


        self.enabled = false;


    }



    pub fn route(

        &self,

        destination:String

    ) {


        if self.enabled {


            println!(
                "Routing {} through Nym",
                destination
            );


        }


    }



    pub fn mixnet_enabled(

        &self

    ) -> bool {


        self.mixnet_mode


    }


}
