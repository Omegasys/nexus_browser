// Nexus MicroVM Lifecycle Manager
// GPL-3.0 License


#[derive(Debug, Clone)]
pub enum VMState {


    Created,

    Starting,

    Running,

    Paused,

    Stopped,

    Destroyed,


}



pub struct VMLifecycle;



impl VMLifecycle {


    pub fn new() -> Self {


        Self {}

    }



    pub fn start(

        &self,

        vm_id:u64

    ) {


        println!(
            "Starting MicroVM {}",
            vm_id
        );


    }



    pub fn pause(

        &self,

        vm_id:u64

    ) {


        println!(
            "Pausing MicroVM {}",
            vm_id
        );


    }



    pub fn resume(

        &self,

        vm_id:u64

    ) {


        println!(
            "Resuming MicroVM {}",
            vm_id
        );


    }



    pub fn shutdown(

        &self,

        vm_id:u64

    ) {


        println!(
            "Stopping MicroVM {}",
            vm_id
        );


    }



}
