//! Proxy Auto Configuration manager.

pub struct PacManager {


    enabled:bool,

    source:
        Option<String>,


}



impl PacManager {


    pub fn new() -> Self {

        Self {

            enabled:false,

            source:
                None,

        }

    }



    pub fn load(
        &mut self,
        source:String
    ) {

        self.source =
            Some(source);

        self.enabled =
            true;

    }



    pub fn enabled(
        &self
    ) -> bool {

        self.enabled

    }

}
