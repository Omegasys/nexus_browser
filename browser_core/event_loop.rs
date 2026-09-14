// Nexus Browser Event Loop
// GPL-3.0 License


use std::collections::VecDeque;



#[derive(Debug)]
pub enum BrowserEvent {


    UserInput(String),

    NetworkEvent(String),

    RenderingEvent(String),

    EngineEvent(String),


}



pub struct EventLoop {


    events:
        VecDeque<BrowserEvent>,


    running:
        bool,


}



impl EventLoop {


    pub fn new() -> Self {


        Self {

            events:
                VecDeque::new(),

            running:
                false,

        }


    }



    pub fn start(
        &mut self
    ) {


        println!(
            "Browser event loop started"
        );


        self.running = true;


    }



    pub fn stop(
        &mut self
    ) {


        self.running = false;


    }



    pub fn push_event(
        &mut self,
        event: BrowserEvent
    ) {


        self.events.push_back(
            event
        );


    }



    pub fn process_events(
        &mut self
    ) {


        while let Some(event) =
            self.events.pop_front()
        {


            println!(
                "Processing event: {:?}",
                event
            );


        }


    }


}
