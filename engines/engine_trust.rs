// Nexus Engine Trust System
// GPL-3.0 License


pub struct EngineTrust {


    trusted_engines:
        Vec<String>,


}



impl EngineTrust {


    pub fn new() -> Self {


        Self {

            trusted_engines:
                Vec::new(),

        }


    }



    pub fn trust(

        &mut self,

        engine:String

    ) {


        println!(
            "Trusting engine {}",
            engine
        );


        self.trusted_engines.push(
            engine
        );


    }



    pub fn revoke(

        &mut self,

        engine:String

    ) {


        self.trusted_engines
            .retain(
                |item|
                item != &engine
            );


    }



    pub fn is_trusted(

        &self,

        engine:String

    ) -> bool {


        self.trusted_engines
            .contains(
                &engine
            )


    }


}
