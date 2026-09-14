// Nexus Engine Health Monitor
// GPL-3.0 License


#[derive(Debug)]
pub enum EngineStatus {


    Healthy,

    Degraded,

    Crashed,

    Disabled,


}



pub struct EngineHealth;



impl EngineHealth {


    pub fn new() -> Self {


        Self {}

    }



    pub fn check(

        &self,

        engine:String

    ) -> EngineStatus {


        println!(
            "Checking health of {}",
            engine
        );


        EngineStatus::Healthy


    }



    pub fn restart_required(

        &self,

        status:EngineStatus

    ) -> bool {


        matches!(
            status,
            EngineStatus::Crashed
        )


    }


}
