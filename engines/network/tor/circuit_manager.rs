//! Tor circuit management.
//!
//! Controls creation and isolation of circuits.

use std::collections::HashMap;



#[derive(Debug, Clone)]

pub struct TorCircuit {


    pub id: String,

    pub purpose: String,

    pub active: bool,


}



pub struct CircuitManager {


    circuits:
        HashMap<String,TorCircuit>,


}



impl CircuitManager {


    pub fn new() -> Self {


        Self {

            circuits:
                HashMap::new(),

        }

    }



    pub fn create_circuit(
        &mut self,
        id: String,
        purpose: String
    ) {


        self.circuits.insert(

            id.clone(),

            TorCircuit {

                id,

                purpose,

                active:
                    true,

            }

        );

    }



    pub fn close_circuit(
        &mut self,
        id: &str
    ) {


        if let Some(
            circuit
        ) =
            self.circuits.get_mut(id)
        {

            circuit.active =
                false;

        }

    }



    pub fn active_count(
        &self
    ) -> usize {


        self.circuits
            .values()
            .filter(
                |c| c.active
            )
            .count()

    }

}


impl Default for CircuitManager {

    fn default() -> Self {

        Self::new()

    }

}
