//! Proxy health monitoring.

pub struct HealthMonitor {


    healthy:bool,


}



impl HealthMonitor {


    pub fn new() -> Self {

        Self {

            healthy:true,

        }

    }



    pub fn status(
        &self
    ) -> bool {

        self.healthy

    }

}
