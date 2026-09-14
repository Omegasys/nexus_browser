// Nexus Kernel Orchestrator
// GPL-3.0 License

use crate::browser_kernel::kernel_core::KernelCore;


pub struct KernelOrchestrator {

    pub kernel: KernelCore,

}


impl KernelOrchestrator {


    pub fn new() -> Self {

        Self {

            kernel: KernelCore::new(),

        }

    }



    pub fn start(&mut self) {

        println!("Starting Nexus Browser");

        self.kernel.initialize();

    }



    pub fn stop(&mut self) {

        println!("Stopping Nexus Browser");

        self.kernel.shutdown();

    }



    pub fn load_default_components(&mut self) {

        println!("Loading default Nexus components");

        self.kernel.register_engine(
            "Blink".to_string()
        );

        self.kernel.register_engine(
            "Gecko".to_string()
        );

        self.kernel.register_engine(
            "Servo".to_string()
        );

    }

}
