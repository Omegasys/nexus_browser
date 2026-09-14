// Nexus Browser Runtime
// GPL-3.0 License


use crate::browser_core::event_loop::EventLoop;
use crate::browser_core::scheduler::Scheduler;


pub struct BrowserRuntime {

    running: bool,

    event_loop: EventLoop,

    scheduler: Scheduler,

}



impl BrowserRuntime {


    pub fn new() -> Self {


        Self {

            running: false,

            event_loop:
                EventLoop::new(),

            scheduler:
                Scheduler::new(),

        }

    }



    pub fn start(
        &mut self
    ) {


        println!(
            "Starting Nexus runtime"
        );


        self.running = true;


        self.event_loop.start();


    }



    pub fn stop(
        &mut self
    ) {


        println!(
            "Stopping Nexus runtime"
        );


        self.running = false;


    }



    pub fn is_running(
        &self
    ) -> bool {


        self.running


    }



    pub fn scheduler(
        &mut self
    ) -> &mut Scheduler {


        &mut self.scheduler


    }


}
