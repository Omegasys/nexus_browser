// Nexus Engine Source Watcher
// GPL-3.0 License


pub struct SourceWatcher {


    watching:bool,


}



impl SourceWatcher {


    pub fn new() -> Self {


        Self {

            watching:false,

        }


    }



    pub fn watch(

        &mut self,

        path:String

    ) {


        println!(
            "Watching source directory {}",
            path
        );


        self.watching=true;


    }



    pub fn source_changed(

        &self,

        file:String

    ) -> bool {


        println!(
            "Checking changes in {}",
            file
        );


        false


    }



}
