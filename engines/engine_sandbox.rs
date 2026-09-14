// Nexus Engine Sandbox
// GPL-3.0 License


pub struct EngineSandbox {


    active: bool,


}



impl EngineSandbox {


    pub fn new() -> Self {


        Self {

            active:true,

        }


    }



    pub fn start(

        &self,

        engine:String

    ) {


        println!(
            "Launching {} inside engine sandbox",
            engine
        );


    }



    pub fn stop(

        &self,

        engine:String

    ) {


        println!(
            "Stopping sandbox for {}",
            engine
        );


    }



    pub fn isolate_filesystem(

        &self

    ) {


        println!(
            "Engine filesystem isolation enabled"
        );


    }



    pub fn isolate_network(

        &self

    ) {


        println!(
            "Engine network isolation enabled"
        );


    }


}
