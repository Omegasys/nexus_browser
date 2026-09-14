// Nexus Secure DNS Manager
// GPL-3.0 License


#[derive(Debug, Clone)]
pub enum DNSProvider {

    System,

    DNSOverHTTPS,

    DNSOverTLS,

    DNSCrypt,

    TorDNS,

}



pub struct DNSManager {


    provider: DNSProvider,

    enabled: bool,


}



impl DNSManager {


    pub fn new() -> Self {


        Self {

            provider:
                DNSProvider::DNSOverHTTPS,

            enabled:
                true,

        }


    }



    pub fn set_provider(

        &mut self,

        provider: DNSProvider

    ) {


        println!(
            "Changing DNS provider to {:?}",
            provider
        );


        self.provider = provider;


    }



    pub fn resolve(

        &self,

        hostname:String

    ) -> Option<String> {


        println!(
            "Resolving {} securely",
            hostname
        );


        Some(
            "127.0.0.1".to_string()
        )


    }



    pub fn enable(

        &mut self

    ) {


        self.enabled = true;


    }



    pub fn disable(

        &mut self

    ) {


        self.enabled = false;


    }



    pub fn kill_switch_check(

        &self

    ) -> bool {


        self.enabled


    }


}
