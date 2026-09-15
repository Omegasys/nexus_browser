//! Browser state partitioning.
//!
//! Separates storage, identifiers,
//! cache, cookies, and sessions.

use std::collections::HashMap;



pub struct StatePartition {


    pub name: String,

    pub storage_id: String,

    pub cookie_id: String,

    pub cache_id: String,

}



pub struct StatePartitionManager {


    partitions:
        HashMap<String, StatePartition>,


    enabled: bool,


}



impl StatePartitionManager {


    pub fn new() -> Self {


        Self {

            partitions:
                HashMap::new(),

            enabled:
                true,

        }

    }



    pub fn create_partition(
        &mut self,
        name: String
    ) {


        let partition =
            StatePartition {

                storage_id:
                    format!(
                        "{}_storage",
                        name
                    ),

                cookie_id:
                    format!(
                        "{}_cookies",
                        name
                    ),

                cache_id:
                    format!(
                        "{}_cache",
                        name
                    ),

                name:
                    name.clone(),

            };


        self.partitions
            .insert(
                name,
                partition
            );

    }



    pub fn remove_partition(
        &mut self,
        name: &str
    ) {


        self.partitions
            .remove(name);

    }



    pub fn get_partition(
        &self,
        name: &str
    ) -> Option<&StatePartition> {

        self.partitions
            .get(name)

    }



    pub fn enabled(
        &self
    ) -> bool {

        self.enabled

    }

}


impl Default for StatePartitionManager {


    fn default() -> Self {

        Self::new()

    }

}
