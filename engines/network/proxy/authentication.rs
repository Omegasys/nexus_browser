//! Proxy authentication manager.

pub struct ProxyAuthentication {


    username:
        Option<String>,


    password:
        Option<String>,


}



impl ProxyAuthentication {


    pub fn new() -> Self {

        Self {

            username:
                None,

            password:
                None,

        }

    }



    pub fn set(
        &mut self,
        username:String,
        password:String
    ) {

        self.username =
            Some(username);

        self.password =
            Some(password);

    }

}
