// Nexus Browser Engine Loader
// GPL-3.0 License

use std::path::PathBuf;


#[derive(Debug, Clone)]
pub struct EnginePackage {

    pub name: String,

    pub version: String,

    pub path: PathBuf,

    pub compiled: bool,

}



pub struct EngineLoader {

    loaded_engines: Vec<EnginePackage>,

}



impl EngineLoader {


    pub fn new() -> Self {

        Self {

            loaded_engines: Vec::new(),

        }

    }



    pub fn scan_engine_directory(
        &self,
        path: PathBuf
    ) {

        println!(
            "Scanning engine directory: {:?}",
            path
        );

    }



    pub fn load_engine(
        &mut self,
        package: EnginePackage
    ) {


        println!(
            "Loading engine: {}",
            package.name
        );


        self.validate_engine(
            &package
        );


        self.loaded_engines.push(
            package
        );

    }



    pub fn unload_engine(
        &mut self,
        name: &str
    ) {


        self.loaded_engines
            .retain(
                |engine|
                engine.name != name
            );


        println!(
            "Engine unloaded: {}",
            name
        );


    }



    fn validate_engine(
        &self,
        package: &EnginePackage
    ) {


        println!(
            "Validating engine {}",
            package.name
        );


        // Future:
        // - Manifest validation
        // - Signature checking
        // - Capability checking
        // - Sandbox testing

    }



    pub fn list_loaded(
        &self
    ) -> Vec<String> {


        self.loaded_engines
            .iter()
            .map(
                |engine|
                engine.name.clone()
            )
            .collect()


    }

}
