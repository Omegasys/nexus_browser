// Nexus MicroVM Rollback System
// GPL-3.0 License


pub struct VMRollback {


    rollback_points:
        Vec<String>,


}



impl VMRollback {


    pub fn new() -> Self {


        Self {

            rollback_points:
                Vec::new(),

        }


    }



    pub fn create_checkpoint(

        &mut self,

        vm_id:u64,

        name:String

    ) {


        let checkpoint =
            format!(
                "VM:{}:{}",
                vm_id,
                name
            );


        println!(
            "Creating rollback checkpoint {}",
            checkpoint
        );


        self.rollback_points.push(
            checkpoint
        );


    }



    pub fn rollback(

        &self,

        vm_id:u64

    ) {


        println!(
            "Rolling back VM {}",
            vm_id
        );


    }



    pub fn rollback_to(

        &self,

        checkpoint:String

    ) {


        println!(
            "Rolling back to {}",
            checkpoint
        );


    }



}
