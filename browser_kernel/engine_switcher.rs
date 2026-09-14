// Nexus Engine Switcher
// GPL-3.0 License

use std::time::SystemTime;


pub struct EngineSwitcher {


    active_engine: Option<String>,


}



impl EngineSwitcher {


    pub fn new() -> Self {

        Self {

            active_engine: None,

        }

    }



    pub fn switch(
        &mut self,
        engine_name: String
    ) {


        println!(
            "Switching active engine to {}",
            engine_name
        );


        self.unload_current();



        self.load_engine(
            engine_name
        );


    }



    fn unload_current(
        &mut self
    ) {


        if let Some(engine) =
            &self.active_engine
        {

            println!(
                "Unloading engine {}",
                engine
            );

        }


        self.active_engine = None;


    }




    fn load_engine(
        &mut self,
        engine_name: String
    ) {


        println!(
            "Loading engine {}",
            engine_name
        );


        self.active_engine =
            Some(engine_name);



        self.verify_engine();


    }



    fn verify_engine(
        &self
    ) {


        println!(
            "Performing engine compatibility check"
        );


        let _timestamp =
            SystemTime::now();


    }



    pub fn current_engine(
        &self
    ) -> Option<String> {


        self.active_engine.clone()


    }

}
