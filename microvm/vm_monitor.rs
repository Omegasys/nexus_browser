// Nexus MicroVM Monitor
// GPL-3.0 License


pub struct VMStatistics {


    pub cpu_usage:f32,

    pub memory_usage:u64,

    pub network_usage:u64,


}



pub struct VMMonitor;



impl VMMonitor {


    pub fn new() -> Self {


        Self {}

    }



    pub fn collect(

        &self,

        vm_id:u64

    ) -> VMStatistics {


        println!(
            "Collecting metrics for VM {}",
            vm_id
        );


        VMStatistics {


            cpu_usage:
                0.0,


            memory_usage:
                0,


            network_usage:
                0,


        }


    }



    pub fn watch(

        &self,

        vm_id:u64

    ) {


        println!(
            "Monitoring VM {}",
            vm_id
        );


    }



}
