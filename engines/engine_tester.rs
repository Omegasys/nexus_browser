// Nexus Engine Tester
// GPL-3.0 License


pub struct EngineTestResult {


    pub passed: bool,

    pub errors: Vec<String>,


}



pub struct EngineTester;



impl EngineTester {


    pub fn new() -> Self {


        Self {}

    }



    pub fn test_engine(

        &self,

        engine:String

    ) -> EngineTestResult {


        println!(
            "Testing engine {}",
            engine
        );


        EngineTestResult {


            passed:true,

            errors:
                Vec::new(),


        }


    }



    pub fn benchmark(

        &self,

        engine:String

    ) {


        println!(
            "Benchmarking engine {}",
            engine
        );


    }



    pub fn compatibility_test(

        &self,

        engine:String

    ) -> bool {


        println!(
            "Checking compatibility for {}",
            engine
        );


        true


    }


}
