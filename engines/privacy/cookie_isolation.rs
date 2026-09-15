//! Cookie isolation system.
//!
//! Separates cookies between sites,
//! profiles, and workspaces.

use std::collections::HashMap;



pub struct CookieIsolation {


    enabled: bool,

    containers:
        HashMap<String, Vec<String>>,


}



impl CookieIsolation {


    pub fn new() -> Self {


        Self {

            enabled: true,

            containers:
                HashMap::new(),

        }

    }



    pub fn create_container(
        &mut self,
        name: String
    ) {

        self.containers
            .entry(name)
            .or_insert(
                Vec::new()
            );

    }



    pub fn add_cookie(
        &mut self,
        container: &str,
        cookie: String
    ) {


        if let Some(
            cookies
        ) =
            self.containers
                .get_mut(container)
        {

            cookies.push(cookie);

        }

    }



    pub fn clear_container(
        &mut self,
        container: &str
    ) {


        if let Some(
            cookies
        ) =
            self.containers
                .get_mut(container)
        {

            cookies.clear();

        }

    }



    pub fn enabled(
        &self
    ) -> bool {

        self.enabled

    }

}


impl Default for CookieIsolation {

    fn default() -> Self {

        Self::new()

    }

}
