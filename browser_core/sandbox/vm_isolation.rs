// Nexus MicroVM Isolation Layer
// GPL-3.0 License


pub struct MicroVM {


    pub id: u64,

    pub running: bool,


}



pub struct VMIsolation {


    machines:
        Vec<MicroVM>,


}



impl VMIsolation {


    pub fn new() -> Self {


        Self {

            machines:
                Vec::new(),

        }


    }



    pub fn create_vm(
        &mut self
    ) -> u64 {


        let id =
            self.machines.len() as u64;



        println!(
            "Creating MicroVM {}",
            id
        );



        self.machines.push(
            MicroVM {

                id,

                running: true,

            }
        );



        id


    }



    pub fn destroy_vm(
        &mut self,
        id: u64
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



    pub fn isolate_engine(
        &self,
        engine: String
    ) {


        println!(
            "Running engine {} inside MicroVM",
            engine
        );


    }


}
