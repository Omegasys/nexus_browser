//! Hyphanet data request manager.

use std::collections::VecDeque;



#[derive(Debug,Clone)]

pub struct HyphanetRequest {


    pub id:String,

    pub target:String,

    pub completed:bool,


}



pub struct RequestManager {


    requests:
        VecDeque<HyphanetRequest>,


}



impl RequestManager {


    pub fn new() -> Self {


        Self {

            requests:
                VecDeque::new(),

        }

    }



    pub fn request(
        &mut self,
        id:String,
        target:String
    ) {


        self.requests.push_back(

            HyphanetRequest {

                id,

                target,

                completed:
                    false,

            }

        );

    }



    pub fn pending(
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
