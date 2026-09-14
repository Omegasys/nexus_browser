// Nexus Engine Rollback System
// GPL-3.0 License


pub struct EngineRollback {


    versions:
        Vec<String>,


}



impl EngineRollback {


    pub fn new() -> Self {


        Self {

            versions:
                Vec::new(),

        }


    }



    pub fn save_version(

        &mut self,

        version:String

    ) {


        println!(
            "Saving engine version {}",
            version
        );


        self.versions.push(
            version
        );


    }



    pub fn rollback(

        &self,

        engine:String

    ) {


        println!(
            "Rolling back {}",
            engine
        );


    }



    pub fn latest(

        &self

    ) -> Option<&String> {


        self.versions.last()


    }


}
