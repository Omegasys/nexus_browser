// Nexus Browser Rendering Pipeline
// GPL-3.0 License


pub struct RenderingPipeline {


    active_engine:
        Option<String>,


}



impl RenderingPipeline {


    pub fn new() -> Self {


        Self {

            active_engine:
                None,

        }


    }



    pub fn attach_engine(
        &mut self,
        engine: String
    ) {


        println!(
            "Attaching rendering engine {}",
            engine
        );


        self.active_engine =
            Some(engine);


    }



    pub fn render_page(
        &self,
        url: String
    ) {


        println!(
            "Rendering page {}",
            url
        );


    }



    pub fn remove_engine(
        &mut self
    ) {


        self.active_engine =
            None;


    }


}
