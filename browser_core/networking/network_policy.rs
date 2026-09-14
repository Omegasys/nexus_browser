// Nexus Network Policy Engine
// GPL-3.0 License


#[derive(Debug, Clone)]
pub enum Protocol {


    IPv4,

    IPv6,

    TCP,

    UDP,

    HTTP,

    HTTPS,

    QUIC,

    DNS,


}



pub struct NetworkPolicy {


    allowed_protocols:
        Vec<Protocol>,


    kill_switch:
        bool,


}



impl NetworkPolicy {


    pub fn new() -> Self {


        Self {

            allowed_protocols:
                vec![

                    Protocol::HTTPS,

                    Protocol::TCP,

                    Protocol::DNS,

                ],


            kill_switch:
                false,


        }


    }



    pub fn allow(

        &mut self,

        protocol:Protocol

    ) {


        self.allowed_protocols.push(
            protocol
        );


    }



    pub fn block(

        &mut self,

        protocol:&Protocol

    ) {


        self.allowed_protocols
            .retain(
                |item|
                std::mem::discriminant(item)
                !=
                std::mem::discriminant(protocol)
            );


    }



    pub fn is_allowed(

        &self,

        protocol:&Protocol

    ) -> bool {


        self.allowed_protocols
            .iter()
            .any(
                |item|
                std::mem::discriminant(item)
                ==
                std::mem::discriminant(protocol)
            )


    }



    pub fn enable_kill_switch(

        &mut self

    ) {


        println!(
            "Network kill switch enabled"
        );


        self.kill_switch = true;


    }



    pub fn disable_kill_switch(

        &mut self

    ) {


        self.kill_switch = false;


    }



    pub fn kill_switch_active(

        &self

    ) -> bool {


        self.kill_switch


    }


}
