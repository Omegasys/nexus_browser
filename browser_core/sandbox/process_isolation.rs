// Nexus Process Isolation System
// GPL-3.0 License


#[derive(Debug, Clone)]
pub struct BrowserProcess {

    pub id: u64,

    pub name: String,

    pub isolated: bool,

}



pub struct ProcessIsolation {


    processes: Vec<BrowserProcess>,


}



impl ProcessIsolation {


    pub fn new() -> Self {

        Self {

            processes:
                Vec::new(),

        }

    }



    pub fn create_process(
        &mut self,
        name: String
    ) -> u64 {


        let id =
            self.processes.len() as u64;



        let process =
            BrowserProcess {

                id,

                name,

                isolated: true,

            };



        println!(
            "Creating isolated process {}",
            id
        );



        self.processes.push(
            process
        );



        id

    }



    pub fn terminate_process(
        &mut self,
        id: u64
    ) {


        self.processes
            .retain(
                |process|
                process.id != id
            );


        println!(
            "Terminated process {}",
            id
        );


    }



    pub fn isolate(
        &self,
        id: u64
    ) -> bool {


        self.processes
            .iter()
            .any(
                |process|
                process.id == id
                &&
                process.isolated
            )


    }


}
