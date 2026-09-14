// Nexus DNS MicroVM
// GPL-3.0 License


#[derive(Debug)]
pub enum DNSMode {


    System,

    DNSOverHTTPS,

    DNSOverTLS,

    DNSCrypt,

    TorDNS,


}



pub struct DNSVM {


    vm_id:u64,

    mode:DNSMode,

    leak_protection:bool,


}



impl DNSVM {


    pub fn new(

        vm_id:u64,

        mode:DNSMode

    ) -> Self {


        Self {

            vm_id,

            mode,

            leak_protection:true,

        }


    }



    pub fn start(

        &self

    ) {


        println!(
            "Starting DNS VM {} using {:?}",
            self.vm_id,
            self.mode
        );


    }



    pub fn resolve(

        &self,

        hostname:String

    ) {


        println!(
            "Resolving {} through isolated DNS VM",
            hostname
        );


    }



    pub fn enable_leak_protection(

        &mut self

    ) {


        self.leak_protection = true;


    }



    pub fn disable_leak_protection(

        &mut self

    ) {


        self.leak_protection = false;


    }



}
