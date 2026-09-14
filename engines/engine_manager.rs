// Nexus Engine Manager
// GPL-3.0 License


use crate::engines::engine_api::BrowserEngine;



pub struct EngineManager {


    engines:
        Vec<Box<dyn BrowserEngine>>,


}



impl EngineManager {


    pub fn new() -> Self {


        Self {

            engines:
                Vec::new(),

        }


    }



    pub fn register_engine(

        &mut self,

        engine:
            Box<dyn BrowserEngine>

    ) {


        println!(
            "Registering engine {}",
            engine.name()
        );


        self.engines.push(
            engine
        );


    }



    pub fn unload_engine(

        &mut self,

        name:String

    ) {


        self.engines
            .retain(
                |engine|
                engine.name()
                !=
                name
            );


    }



    pub fn list_engines(

        &self

    ) -> Vec<String> {


        self.engines
            .iter()
            .map(
                |engine|
                engine.name()
            )
            .collect()


    }



}
