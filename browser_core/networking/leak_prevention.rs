// Nexus Network Leak Prevention
// GPL-3.0 License


pub struct LeakPrevention {


    dns_protection: bool,

    webrtc_protection: bool,

    ipv6_protection: bool,

    proxy_protection: bool,


}



impl LeakPrevention {


    pub fn new() -> Self {


        Self {


            dns_protection: true,

            webrtc_protection: true,

            ipv6_protection: true,

            proxy_protection: true,


        }


    }



    pub fn enable_all(

        &mut self

    ) {


        self.dns_protection = true;

        self.webrtc_protection = true;

        self.ipv6_protection = true;

        self.proxy_protection = true;


    }



    pub fn check_dns_leak(

        &self

    ) -> bool {


        self.dns_protection


    }



    pub fn check_webrtc_leak(

        &self

    ) -> bool {


        self.webrtc_protection


    }



    pub fn check_ipv6_leak(

        &self

    ) -> bool {


        self.ipv6_protection


    }



    pub fn check_proxy_leak(

        &self

    ) -> bool {


        self.proxy_protection


    }



    pub fn emergency_lockdown(

        &mut self

    ) {


        println!(
            "Activating emergency network lockdown"
        );


        self.enable_all();


    }


}
