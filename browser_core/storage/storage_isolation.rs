// Nexus Storage Isolation
// GPL-3.0 License


pub struct StorageIsolation {


    cookie_isolation: bool,

    cache_isolation: bool,

    workspace_isolation: bool,


}



impl StorageIsolation {


    pub fn new() -> Self {


        Self {


            cookie_isolation: true,

            cache_isolation: true,

            workspace_isolation: true,


        }


    }



    pub fn enforce(

        &self

    ) {


        println!(
            "Enforcing storage isolation"
        );


    }



    pub fn cookie_isolated(

        &self

    ) -> bool {


        self.cookie_isolation


    }



    pub fn cache_isolated(

        &self

    ) -> bool {


        self.cache_isolation


    }


}
