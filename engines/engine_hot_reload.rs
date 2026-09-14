// Nexus Engine Hot Reload System
// GPL-3.0 License


pub struct EngineHotReload {


    enabled:bool,


}



impl EngineHotReload {


    pub fn new() -> Self {


        Self {

            enabled:true,

        }


    }



    pub fn reload(

        &self,

        engine:String

    ) {


        if self.enabled {


            println!(
                "Hot reloading engine {}",
                engine
            );


        }


    }



    pub fn enable(

        &mut self

    ) {


        self.enabled=true;


    }



    pub fn disable(

        &mut self

    ) {


        self.enabled=false;


    }



}
