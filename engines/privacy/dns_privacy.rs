//! DNS privacy subsystem.
//!
//! Controls encrypted DNS routing.

#[derive(Debug, Clone)]
pub enum DnsMode {

    System,

    DNSOverHTTPS,

    DNSOverTLS,

    Tor,

    I2P,

    Custom,

}



pub struct DnsPrivacy {


    enabled: bool,

    mode: DnsMode,

    provider: Option<String>,


}



impl DnsPrivacy {


    pub fn new() -> Self {


        Self {

            enabled: true,

            mode:
                DnsMode::DNSOverHTTPS,

            provider:
                None,

        }

    }



    pub fn set_mode(
        &mut self,
        mode: DnsMode
    ) {

        self.mode = mode;

    }



    pub fn set_provider(
        &mut self,
        provider: String
    ) {

        self.provider =
            Some(provider);

    }



    pub fn mode(
        &self
    ) -> &DnsMode {

        &self.mode

    }



    pub fn enabled(
        &self
    ) -> bool {

        self.enabled

    }

}


impl Default for DnsPrivacy {

    fn default() -> Self {

        Self::new()

    }

}
