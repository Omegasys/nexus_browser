// Nexus IPC Message Bus
// GPL-3.0 License


use std::collections::VecDeque;



pub struct IPCMessage {


    pub sender: String,

    pub receiver: String,

    pub data: String,


}



pub struct IPCMessageBus {


    queue:
        VecDeque<IPCMessage>,


}



impl IPCMessageBus {


    pub fn new() -> Self {


        Self {

            queue:
                VecDeque::new(),

        }


    }



    pub fn send(
        &mut self,
        message: IPCMessage
    ) {


        self.queue.push_back(
            message
        );


    }



    pub fn receive(
        &mut self
    ) -> Option<IPCMessage> {


        self.queue.pop_front()


    }


}
