// Nexus Transport Filter
// GPL-3.0 License


#[derive(Debug)]
pub enum Transport {


    TCP,

    UDP,

    QUIC,


}



pub struct TransportFilter {


    tcp_enabled: bool,

    udp_enabled: bool,

    quic_enabled: bool,


}



impl TransportFilter {


    pub fn new() -> Self {


        Self {


            tcp_enabled: true,

            udp_enabled: true,

            quic_enabled: true,


        }


    }



    pub fn set_tcp(

        &mut self,

        enabled:bool

    ) {


        self.tcp_enabled = enabled;


    }



    pub fn set_udp(

        &mut self,

        enabled:bool

    ) {


        self.udp_enabled = enabled;


    }



    pub fn set_quic(

        &mut self,

        enabled:bool

    ) {


        self.quic_enabled = enabled;


    }



    pub fn allowed(

        &self,

        transport:Transport

    ) -> bool {


        match transport {


            Transport::TCP =>
                self.tcp_enabled,


            Transport::UDP =>
                self.udp_enabled,


            Transport::QUIC =>
                self.quic_enabled,


        }


    }


}
