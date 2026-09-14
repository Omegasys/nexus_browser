// Nexus Browser Inter Engine Communication Bus
// GPL-3.0 License


use std::collections::VecDeque;


#[derive(Debug, Clone)]
pub struct EngineMessage {

    pub sender: String,

    pub receiver: String,

    pub message_type: String,

    pub payload: String,

}



pub struct InterEngineBus {


    queue: VecDeque<EngineMessage>,


}



impl InterEngineBus {


    pub fn new() -> Self {


        Self {

            queue: VecDeque::new(),

        }


    }



    pub fn send(
        &mut self,
        message: EngineMessage
    ) {


        println!(
            "Message from {} to {}",
            message.sender,
            message.receiver
        );


        self.queue.push_back(
            message
        );


    }



    pub fn receive(
        &mut self
    ) -> Option<EngineMessage> {


        self.queue.pop_front()


    }



    pub fn validate_message(
        &self,
        message: &EngineMessage
    ) -> bool {


        println!(
            "Validating message {}",
            message.message_type
        );


        true

    }

}
