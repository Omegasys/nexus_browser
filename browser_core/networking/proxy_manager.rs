// Nexus Proxy Manager
// GPL-3.0 License


#[derive(Debug, Clone)]
pub enum ProxyType {


    None,

    HTTP,

    SOCKS5,

    VPN,

    Tor,


}



pub struct ProxyManager {


    active_proxy:
        ProxyType,


}



impl ProxyManager {


    pub fn new() -> Self {


        Self {

            active_proxy:
                ProxyType::None,

        }


    }



    pub fn set_proxy(

        &mut self,

        proxy:ProxyType

    ) {


        println!(
            "Setting proxy {:?}",
            proxy
        );


        self.active_proxy =
            proxy;


    }



    pub fn remove_proxy(

        &mut self

    ) {


        self.active_proxy =
            ProxyType::None;


    }



    pub fn current(

        &self

    ) -> &ProxyType {


        &self.active_proxy


    }


}
