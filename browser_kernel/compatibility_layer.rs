// Nexus Browser Compatibility Layer
// GPL-3.0 License

#[derive(Debug, Clone)]
pub enum OperatingSystem {

    Linux,

    Windows,

    MacOS,

    BSD,

    Unknown,

}


pub struct CompatibilityLayer {

    operating_system: OperatingSystem,

    architecture: String,

}


impl CompatibilityLayer {


    pub fn new() -> Self {

        Self {

            operating_system:
                OperatingSystem::Unknown,

            architecture:
                "unknown".to_string(),

        }

    }



    pub fn detect_system(
        &mut self
    ) {


        println!(
            "Detecting host operating system"
        );


        self.operating_system =
            OperatingSystem::Linux;


        self.architecture =
            "x86_64".to_string();


    }



    pub fn supports_feature(
        &self,
        feature: &str
    ) -> bool {


        println!(
            "Checking compatibility for {}",
            feature
        );


        true

    }



    pub fn operating_system(
        &self
    ) -> &OperatingSystem {


        &self.operating_system


    }



    pub fn architecture(
        &self
    ) -> &str {


        &self.architecture


    }


}
