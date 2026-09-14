// Nexus State Partitioning System
// GPL-3.0 License


#[derive(Clone)]
pub struct StoragePartition {


    pub workspace: String,

    pub site: String,

}



pub struct StatePartitioning {


    partitions:
        Vec<StoragePartition>,


}



impl StatePartitioning {


    pub fn new() -> Self {


        Self {

            partitions:
                Vec::new(),

        }


    }



    pub fn create_partition(

        &mut self,

        workspace: String,

        site: String

    ) {


        println!(
            "Creating storage partition {} {}",
            workspace,
            site
        );


        self.partitions.push(

            StoragePartition {

                workspace,

                site,

            }

        );


    }



    pub fn isolate_site(

        &self,

        site: String

    ) {


        println!(
            "Isolating state for {}",
            site
        );


    }



    pub fn remove_partition(

        &mut self,

        site: String

    ) {


        self.partitions
            .retain(
                |partition|
                partition.site != site
            );


    }


}
