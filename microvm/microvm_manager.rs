// Nexus MicroVM Manager
// GPL-3.0 License


use crate::microvm::vm_lifecycle::VMState;


pub struct MicroVM {


    pub id: u64,

    pub name: String,

    pub state: VMState,

    pub memory_mb: u64,

    pub cpu_count: u8,


}



pub struct MicroVMManager {


    machines:
        Vec<MicroVM>,


}



impl MicroVMManager {


    pub fn new() -> Self {


        Self {

            machines:
                Vec::new(),

        }


    }



    pub fn create_vm(

        &mut self,

        name:String

    ) -> u64 {


        let id =
            self.machines.len() as u64;



        println!(
            "Creating MicroVM {}",
            name
        );



        self.machines.push(

            MicroVM {

                id,

                name,

                state:
                    VMState::Created,

                memory_mb:
                    512,

                cpu_count:
                    1,

            }

        );


        id


    }



    pub fn destroy_vm(

        &mut self,

        id:u64

    ) {


        self.machines
            .retain(
                |vm|
                vm.id != id
            );


        println!(
            "Destroyed MicroVM {}",
            id
        );


    }



    pub fn get_vm(

        &self,

        id:u64

    ) -> Option<&MicroVM> {


        self.machines
            .iter()
            .find(
                |vm|
                vm.id == id
            )


    }



    pub fn list_vms(

        &self

    ) -> usize {


        self.machines.len()


    }


}
