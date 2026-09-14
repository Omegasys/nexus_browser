// Nexus Renderer MicroVM
// GPL-3.0 License


#[derive(Debug)]
pub enum RenderingEngine {


    Blink,

    Gecko,

    Servo,


}



pub struct RendererVM {


    vm_id:u64,

    engine:
        RenderingEngine,


}



impl RendererVM {


    pub fn new(

        vm_id:u64,

        engine:RenderingEngine

    ) -> Self {


        Self {


            vm_id,

            engine,


        }


    }



    pub fn start(

        &self

    ) {


        println!(
            "Starting renderer VM {} using {:?}",
            self.vm_id,
            self.engine
        );


    }



    pub fn shutdown(

        &self

    ) {


        println!(
            "Stopping renderer VM {}",
            self.vm_id
        );


    }



    pub fn isolate_renderer(

        &self

    ) {


        println!(
            "Renderer isolation enabled"
        );


    }



}
