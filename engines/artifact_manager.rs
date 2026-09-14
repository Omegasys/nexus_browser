// Nexus Engine Artifact Manager
// GPL-3.0 License


pub struct EngineArtifact {


    pub name:String,

    pub location:String,

    pub checksum:String,


}



pub struct ArtifactManager {


    artifacts:
        Vec<EngineArtifact>,


}



impl ArtifactManager {


    pub fn new() -> Self {


        Self {

            artifacts:
                Vec::new(),

        }


    }



    pub fn register(

        &mut self,

        artifact:EngineArtifact

    ) {


        println!(
            "Registering artifact {}",
            artifact.name
        );


        self.artifacts.push(
            artifact
        );


    }



    pub fn remove(

        &mut self,

        name:String

    ) {


        self.artifacts
            .retain(
                |item|
                item.name != name
            );


    }



    pub fn list(

        &self

    ) -> usize {


        self.artifacts.len()


    }


}
