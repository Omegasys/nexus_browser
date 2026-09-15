//! Freenet request manager.
//!
//! Handles distributed data requests.

use std::collections::VecDeque;



#[derive(Debug,Clone)]

pub struct FreenetRequest {


    pub id:String,

    pub target:String,

    pub completed:bool,


}



pub struct RequestManager {


    requests:
        VecDeque<FreenetRequest>,


}



impl RequestManager {


    pub fn new() -> Self {


        Self {

            requests:
                VecDeque::new(),

        }

    }



    pub fn add_request(
        &mut self,
        id:String,
        target:String
    ) {


        self.requests
            .push_back(

                FreenetRequest {

                    id,

                    target,

                    completed:
                        false,

                }

            );

    }



    pub fn pending_count(
        &self
    ) -> usize {


        self.requests
            .iter()
            .filter(
                |r|
                !r.completed
            )
            .count()

    }

}



impl Default for RequestManager {


    fn default() -> Self {

        Self::new()

    }

}
