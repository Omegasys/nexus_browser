// Nexus Browser Resource Manager
// GPL-3.0 License


pub struct ResourceManager {


    cpu_limit: u32,

    memory_limit: u64,

    gpu_enabled: bool,


}



impl ResourceManager {


    pub fn new() -> Self {


        Self {

            cpu_limit: 100,

            memory_limit: 4096,

            gpu_enabled: true,

        }


    }



    pub fn allocate_cpu(
        &self,
        amount: u32
    ) {


        println!(
            "Allocating {}% CPU",
            amount
        );


    }



    pub fn allocate_memory(
        &self,
        amount: u64
    ) {


        println!(
            "Allocating {} MB RAM",
            amount
        );


    }



    pub fn enable_gpu(
        &mut self
    ) {


        self.gpu_enabled = true;


    }



    pub fn disable_gpu(
        &mut self
    ) {


        self.gpu_enabled = false;


    }



    pub fn status(
        &self
    ) {


        println!(
            "CPU limit: {}%",
            self.cpu_limit
        );


        println!(
            "Memory limit: {} MB",
            self.memory_limit
        );


    }

}
