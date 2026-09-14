// Nexus Network MicroVM
// GPL-3.0 License


#[derive(Debug)]
pub enum NetworkMode {


    Direct,

    VPN,

    Tor,

    I2P,

    Nym,

    Lokinet,


}



pub struct NetworkVM {


    vm_id:u64,

    mode:NetworkMode,

    isolated:bool,


}



impl NetworkVM {


    pub fn new(

        vm_id:u64,

        mode:NetworkMode

    ) -> Self {


        Self {

            vm_id,

            mode,

            isolated:true,

        }


    }



    pub fn start(

        &self

    ) {


        println!(
            "Starting network VM {} using {:?}",
            self.vm_id,
            self.mode
        );


    }



    pub fn stop(

        &self

    ) {


        println!(
            "Stopping network VM {}",
            self.vm_id
        );


    }



    pub fn isolate(

        &mut self

    ) {


        println!(
            "Network isolation enabled"
        );


        self.isolated = true;


    }



    pub fn is_isolated(

        &self

    ) -> bool {


        self.isolated


    }



}
