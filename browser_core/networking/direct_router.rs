// Nexus Direct Network Router
// GPL-3.0 License


pub struct DirectRouter {


    enabled: bool,



}



impl DirectRouter {


    pub fn new() -> Self {


        Self {

            enabled: true,

        }


    }



    pub fn enable(

        &mut self

    ) {


        println!(
            "Using direct network connection"
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
                "Direct connection to {}",
                destination
            );


        }


    }



}
