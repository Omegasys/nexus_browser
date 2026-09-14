// Nexus Browser Kernel Core
// GPL-3.0 License

use crate::browser_kernel::engine_registry::EngineRegistry;
use crate::browser_kernel::engine_switcher::EngineSwitcher;

pub struct KernelCore {
    pub engine_registry: EngineRegistry,
    pub engine_switcher: EngineSwitcher,
    running: bool,
}


impl KernelCore {

    pub fn new() -> Self {

        Self {
            engine_registry: EngineRegistry::new(),
            engine_switcher: EngineSwitcher::new(),
            running: false,
        }

    }


    pub fn initialize(&mut self) {

        println!("Initializing Nexus Browser Kernel");

        self.running = true;

    }


    pub fn shutdown(&mut self) {

        println!("Shutting down Nexus Kernel");

        self.running = false;

    }


    pub fn is_running(&self) -> bool {

        self.running

    }


    pub fn register_engine(
        &mut self,
        engine_name: String
    ) {

        self.engine_registry.register(
            engine_name
        );

    }


    pub fn switch_engine(
        &mut self,
        engine_name: String
    ) {

        self.engine_switcher.switch(
            engine_name
        );

    }

}
