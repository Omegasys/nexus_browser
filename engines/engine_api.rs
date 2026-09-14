// Nexus Browser Engine API
// GPL-3.0 License


#[derive(Debug, Clone)]
pub enum EngineType {


    Rendering,

    Javascript,

    Network,

    Security,

    Privacy,


}



pub trait BrowserEngine {


    fn name(
        &self
    ) -> String;



    fn version(
        &self
    ) -> String;



    fn initialize(
        &mut self
    );



    fn shutdown(
        &mut self
    );



    fn capabilities(
        &self
    ) -> Vec<String>;


}
