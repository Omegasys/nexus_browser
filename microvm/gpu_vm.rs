// Nexus GPU MicroVM
// GPL-3.0 License


#[derive(Debug)]
pub enum GPUMode {


    Software,

    VirtualGPU,

    HardwareAccelerated,


}



pub struct GPUVM {


    vm_id:u64,

    mode:GPUMode,

    isolated:bool,


}



impl GPUVM {


    pub fn new(

        vm_id:u64,

        mode:GPUMode

    ) -> Self {


        Self {

            vm_id,

            mode,

            isolated:true,

        }


    }



    pub fn start(

        &self

    ) {


        println!(
            "Starting GPU VM {} using {:?}",
            self.vm_id,
            self.mode
        );


    }



    pub fn stop(

        &self

    ) {


        println!(
            "Stopping GPU VM {}",
            self.vm_id
        );


    }



    pub fn enable_isolation(

        &mut self

    ) {


        self.isolated = true;


    }



    pub fn isolated(

        &self

    ) -> bool {


        self.isolated


    }



}
