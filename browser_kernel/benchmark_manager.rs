// Nexus Browser Benchmark Manager
// GPL-3.0 License


#[derive(Debug)]
pub struct BenchmarkResult {


    pub name: String,

    pub score: f64,


}



pub struct BenchmarkManager {


    results:
        Vec<BenchmarkResult>,


}



impl BenchmarkManager {


    pub fn new() -> Self {


        Self {

            results:
                Vec::new(),

        }


    }



    pub fn run_benchmark(
        &mut self,
        name: String
    ) {


        println!(
            "Running benchmark {}",
            name
        );


        let result =
            BenchmarkResult {

                name,

                score: 0.0,

            };


        self.results.push(
            result
        );


    }



    pub fn privacy_score(
        &self
    ) -> f64 {


        0.0


    }



    pub fn fingerprint_score(
        &self
    ) -> f64 {


        0.0


    }



    pub fn isolation_score(
        &self
    ) -> f64 {


        0.0


    }



}
