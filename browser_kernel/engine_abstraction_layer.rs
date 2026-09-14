// Nexus Engine Abstraction Layer
// GPL-3.0 License


#[derive(Clone, Debug)]
pub enum EngineType {

    Rendering,

    Javascript,

    Network,

    Security,

    Privacy,

    AI,

}



#[derive(Clone, Debug)]
pub struct EngineMetadata {

    pub name: String,

    pub version: String,

    pub engine_type: EngineType,

    pub hot_swappable: bool,

}



pub trait Engine {


    fn metadata(
        &self
    ) -> EngineMetadata;



    fn initialize(
        &mut self
    );



    fn shutdown(
        &mut self
    );



    fn health_check(
        &self
    ) -> bool;



}
