// Nexus Browser System Metrics
// GPL-3.0 License


pub struct SystemMetrics {


    cpu_usage: f32,

    memory_usage: u64,

    network_usage: u64,

    gpu_usage: f32,


}



impl SystemMetrics {


    pub fn new() -> Self {


        Self {


            cpu_usage: 0.0,

            memory_usage: 0,

            network_usage: 0,

            gpu_usage: 0.0,


        }


    }



    pub fn update(
        &mut self
    ) {


        println!(
            "Updating system metrics"
        );


    }



    pub fn cpu(
        &self
    ) -> f32 {


        self.cpu_usage


    }



    pub fn memory(
        &self
    ) -> u64 {


        self.memory_usage


    }



    pub fn network(
        &self
    ) -> u64 {


        self.network_usage


    }



    pub fn gpu(
        &self
    ) -> f32 {


        self.gpu_usage


    }


}
