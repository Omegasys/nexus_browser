// Nexus MicroVM Launcher
// GPL-3.0 License


pub struct VMLauncher {


    hypervisor:
        String,


}



impl VMLauncher {


    pub fn new() -> Self {


        Self {

            hypervisor:
                "KVM".to_string(),

        }


    }



    pub fn launch(

        &self,

        vm_id:u64

    ) {


        println!(
            "Launching MicroVM {} using {}",
            vm_id,
            self.hypervisor
        );


    }



    pub fn configure_memory(

        &self,

        vm_id:u64,

        memory:u64

    ) {


        println!(
            "Assigning {}MB memory to VM {}",
            memory,
            vm_id
        );


    }



    pub fn configure_cpu(

        &self,

        vm_id:u64,

        cores:u8

    ) {


        println!(
            "Assigning {} CPU cores to VM {}",
            cores,
            vm_id
        );


    }



}
