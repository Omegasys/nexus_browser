// Nexus Capability Permission Model
// GPL-3.0 License


#[derive(Debug, Clone)]
pub enum Capability {


    NetworkAccess,

    FileAccess,

    Camera,

    Microphone,

    USB,

    Bluetooth,

    GPU,

    Storage,

}



pub struct CapabilityModel {


    permissions:
        Vec<Capability>,


}



impl CapabilityModel {


    pub fn new() -> Self {


        Self {

            permissions:
                Vec::new(),

        }


    }



    pub fn grant(
        &mut self,
        capability: Capability
    ) {


        println!(
            "Granting capability {:?}",
            capability
        );


        self.permissions.push(
            capability
        );


    }



    pub fn revoke(
        &mut self,
        capability: Capability
    ) {


        self.permissions
            .retain(
                |item|
                std::mem::discriminant(item)
                !=
                std::mem::discriminant(&capability)
            );


    }



    pub fn allowed(
        &self,
        capability: Capability
    ) -> bool {


        self.permissions
            .iter()
            .any(
                |item|
                std::mem::discriminant(item)
                ==
                std::mem::discriminant(&capability)
            )


    }


}
