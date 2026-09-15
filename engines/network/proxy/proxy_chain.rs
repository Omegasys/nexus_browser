//! Multi-hop proxy chaining.

use super::proxy_config::ProxyConfig;


pub struct ProxyChain {


    proxies:
        Vec<ProxyConfig>,


}



impl ProxyChain {


    pub fn new() -> Self {

        Self {

            proxies:
                Vec::new(),

        }

    }



    pub fn add(
        &mut self,
        proxy:ProxyConfig
    ) {

        self.proxies.push(proxy);

    }



    pub fn length(
        &self
    ) -> usize {

        self.proxies.len()

    }

}
