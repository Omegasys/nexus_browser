// Nexus Browser Blink Adapter
// GPL-3.0 License

use std::path::PathBuf;


#[derive(Debug, Clone)]
pub struct BlinkConfig {

    pub source_path: PathBuf,

    pub build_path: PathBuf,

    pub artifact_path: PathBuf,

    pub sandboxed: bool,

    pub microvm_required: bool,

}


pub struct BlinkAdapter {

    config: BlinkConfig,

    loaded: bool,

}


impl BlinkAdapter {


    pub fn new(

        config: BlinkConfig

    ) -> Self {

        Self {

            config,

            loaded: false,

        }

    }


    pub fn load(&mut self) -> Result<(), String> {

        if self.config.microvm_required {

            println!(
                "Blink requires MicroVM isolation"
            );

        }


        println!(
            "Loading Blink from {:?}",
            self.config.artifact_path
        );


        self.loaded = true;


        Ok(())

    }


    pub fn unload(&mut self) {

        println!(
            "Unloading Blink"
        );


        self.loaded = false;

    }


    pub fn reload(&mut self) -> Result<(), String> {

        self.unload();

        self.load()

    }


    pub fn is_loaded(&self) -> bool {

        self.loaded

    }


    pub fn source_path(&self) -> &PathBuf {

        &self.config.source_path

    }


    pub fn build_path(&self) -> &PathBuf {

        &self.config.build_path

    }


    pub fn artifact_path(&self) -> &PathBuf {

        &self.config.artifact_path

    }

}
