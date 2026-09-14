// Nexus Engine Builder
// GPL-3.0 License


pub struct EngineBuilder {


}



impl EngineBuilder {


    pub fn new() -> Self {


        Self {}

    }



    pub fn build(

        &self,

        engine_name:String

    ) -> bool {


        println!(
            "Building engine {}",
            engine_name
        );


        true


    }



    pub fn package(

        &self,

        engine_name:String

    ) {


        println!(
            "Packaging engine {}",
            engine_name
        );


    }



}
