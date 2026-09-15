//! Proxy configuration.

#[derive(Debug,Clone)]

pub enum ProxyType {

    HTTP,

    HTTPS,

    SOCKS4,

    SOCKS5,

}



pub struct ProxyConfig {


    pub address:String,

    pub port:u16,

    pub proxy_type:ProxyType,

    pub enabled:bool,


}



impl ProxyConfig {


    pub fn new(
        address:String,
        port:u16,
        proxy_type:ProxyType
    ) -> Self {

        Self {

            address,

            port,

            proxy_type,

            enabled:true,

        }

    }

}
