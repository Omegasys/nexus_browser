//! Connection state tracking.


pub struct StateTracker {


    connections:u64,


}



impl StateTracker {


    pub fn new() -> Self {

        Self {

            connections:0,

        }

    }



    pub fn count(
        &self
    ) -> u64 {

        self.connections

    }

}
