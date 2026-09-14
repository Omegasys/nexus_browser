// Nexus Protocol Filter
// GPL-3.0 License


#[derive(Debug, Clone)]
pub enum Protocol {


    HTTP,

    HTTPS,

    DNS,

    WebRTC,

    QUIC,

    FTP,

    SMTP,

    Custom(String),


}



pub struct ProtocolFilter {


    blocked:
        Vec<Protocol>,


}



impl ProtocolFilter {


    pub fn new() -> Self {


        Self {

            blocked:
                Vec::new(),

        }


    }



    pub fn block(

        &mut self,

        protocol:Protocol

    ) {


        println!(
            "Blocking protocol {:?}",
            protocol
        );


        self.blocked.push(
            protocol
        );


    }



    pub fn allow(

        &mut self,

        protocol:&Protocol

    ) {


        self.blocked
            .retain(
                |item|
                std::mem::discriminant(item)
                !=
                std::mem::discriminant(protocol)
            );


    }



    pub fn is_blocked(

        &self,

        protocol:&Protocol

    ) -> bool {


        self.blocked
            .iter()
            .any(
                |item|
                std::mem::discriminant(item)
                ==
                std::mem::discriminant(protocol)
            )


    }


}
