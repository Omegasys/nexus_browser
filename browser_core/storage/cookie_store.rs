// Nexus Cookie Store
// GPL-3.0 License


use std::collections::HashMap;


pub struct Cookie {


    pub name: String,

    pub value: String,

    pub domain: String,


}



pub struct CookieStore {


    cookies:
        HashMap<String, Vec<Cookie>>,


}



impl CookieStore {


    pub fn new() -> Self {


        Self {

            cookies:
                HashMap::new(),

        }


    }



    pub fn set_cookie(

        &mut self,

        cookie: Cookie

    ) {


        println!(
            "Setting cookie {}",
            cookie.name
        );


        self.cookies
            .entry(cookie.domain.clone())
            .or_insert(Vec::new())
            .push(cookie);


    }



    pub fn get_cookies(

        &self,

        domain: &str

    ) -> Option<&Vec<Cookie>> {


        self.cookies.get(domain)


    }



    pub fn delete_site_cookies(

        &mut self,

        domain: &str

    ) {


        self.cookies.remove(domain);


    }



    pub fn delete_all(

        &mut self

    ) {


        self.cookies.clear();


    }


}
