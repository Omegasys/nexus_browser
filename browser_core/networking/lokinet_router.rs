// Nexus Lokinet Router
// GPL-3.0 License


pub struct LokinetRouter {


    enabled: bool,


}



impl LokinetRouter {


    pub fn new() -> Self {


        Self {

            enabled: false,

        }


    }



    pub fn enable(

        &mut self

    ) {


        println!(
            "Enabling Lokinet routing"
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
                "Routing {} through Lokinet",
                destination
            );


        }


    }



    pub fn status(

        &self

    ) -> bool {


        self.enabled


    }


}
