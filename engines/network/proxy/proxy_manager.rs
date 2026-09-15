//! Proxy manager.

use super::proxy_config::ProxyConfig;


pub struct ProxyManager {


    proxies:
        Vec<ProxyConfig>,


}



impl ProxyManager {


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



    pub fn count(
        &self
    ) -> usize {

        self.proxies.len()

    }

}



impl Default for ProxyManager {

    fn default() -> Self {

        Self::new()

    }

}
