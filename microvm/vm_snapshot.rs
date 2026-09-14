// Nexus MicroVM Snapshot System
// GPL-3.0 License


pub struct VMSnapshot {


    snapshots:
        Vec<String>,


}



impl VMSnapshot {


    pub fn new() -> Self {


        Self {

            snapshots:
                Vec::new(),

        }


    }



    pub fn create_snapshot(

        &mut self,

        vm_id:u64,

        name:String

    ) {


        println!(
            "Creating snapshot {} for VM {}",
            name,
            vm_id
        );



        self.snapshots.push(
            name
        );


    }



    pub fn delete_snapshot(

        &mut self,

        name:&str

    ) {


        self.snapshots
            .retain(
                |snapshot|
                snapshot != name
            );


    }



    pub fn list_snapshots(

        &self

    ) -> Vec<String> {


        self.snapshots.clone()


    }


}
