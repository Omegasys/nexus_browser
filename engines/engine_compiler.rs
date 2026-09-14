// Nexus Engine Compiler
// GPL-3.0 License


pub struct EngineCompiler {


    compiler_path:String,


}



impl EngineCompiler {


    pub fn new(

        compiler:String

    ) -> Self {


        Self {

            compiler_path:
                compiler,

        }


    }



    pub fn compile(

        &self,

        source:String

    ) -> bool {


        println!(
            "Compiling engine source {} using {}",
            source,
            self.compiler_path
        );


        true


    }



    pub fn supported_languages(

        &self

    ) -> Vec<String> {


        vec![

            "Rust".to_string(),

            "C".to_string(),

            "C++".to_string(),

            "JSON".to_string(),

        ]


    }



}
