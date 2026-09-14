// Nexus Engine Manifest
// GPL-3.0 License


#[derive(Debug, Clone)]
pub struct EngineManifest {


    pub name:String,


    pub version:String,


    pub author:String,


    pub engine_type:String,


    pub source_path:String,


    pub compiler:String,


    pub hot_reload:bool,


}



impl EngineManifest {


    pub fn new(

        name:String,

        version:String

    ) -> Self {


        Self {


            name,

            version,

            author:
                "Unknown".to_string(),

            engine_type:
                "Rendering".to_string(),

            source_path:
                String::new(),

            compiler:
                String::new(),

            hot_reload:
                false,


        }


    }



    pub fn enable_hot_reload(

        &mut self

    ) {


        self.hot_reload = true;


    }



}
