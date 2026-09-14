// Nexus I2P Router
// GPL-3.0 License


pub struct I2PRouter {


    enabled: bool,


}



impl I2PRouter {


    pub fn new() -> Self {


        Self {

            enabled: false,

        }


    }



    pub fn enable(

        &mut self

    ) {


        println!(
            "Starting I2P routing"
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
                "Routing {} through I2P",
                destination
            );


        }


    }



    pub fn enabled(

        &self

    ) -> bool {


        self.enabled


    }


}
