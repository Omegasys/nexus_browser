// Nexus Engine Registry
// GPL-3.0 License

use std::collections::HashMap;


pub struct EngineRegistry {


    engines: HashMap<String, String>,


}



impl EngineRegistry {


    pub fn new() -> Self {

        Self {

            engines: HashMap::new(),

        }

    }



    pub fn register(
        &mut self,
        engine_name: String
    ) {


        println!(
            "Registering engine: {}",
            engine_name
        );


        self.engines.insert(
            engine_name.clone(),
            "available".to_string()
        );


    }



    pub fn remove(
        &mut self,
        engine_name: &str
    ) {


        self.engines.remove(
            engine_name
        );


    }



    pub fn exists(
        &self,
        engine_name: &str
    ) -> bool {


        self.engines.contains_key(
            engine_name
        )


    }



    pub fn list(
        &self
    ) -> Vec<String> {


        self.engines
            .keys()
            .cloned()
            .collect()


    }


}
