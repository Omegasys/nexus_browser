// Nexus MicroVM Health System
// GPL-3.0 License


#[derive(Debug)]
pub enum HealthStatus {


    Healthy,

    Warning,

    Critical,

    Offline,


}



pub struct VMHealth;



impl VMHealth {


    pub fn new() -> Self {


        Self {}

    }



    pub fn check(

        &self,

        vm_id:u64

    ) -> HealthStatus {


        println!(
            "Checking VM health {}",
            vm_id
        );


        HealthStatus::Healthy


    }



    pub fn repair_needed(

        &self,

        status:HealthStatus

    ) -> bool {


        match status {


            HealthStatus::Critical =>
                true,


            HealthStatus::Offline =>
                true,


            _ =>
                false,


        }


    }



}
