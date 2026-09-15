//! Packet classification.


#[derive(Debug)]

pub enum PacketType {

    TCP,

    UDP,

    ICMP,

    Unknown,

}



pub struct PacketClassifier;



impl PacketClassifier {


    pub fn classify(
        protocol:&str
    ) -> PacketType {


        match protocol {


            "TCP" =>
                PacketType::TCP,


            "UDP" =>
                PacketType::UDP,


            "ICMP" =>
                PacketType::ICMP,


            _ =>
                PacketType::Unknown,


        }

    }

}
