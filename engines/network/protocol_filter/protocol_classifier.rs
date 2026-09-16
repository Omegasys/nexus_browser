//! Classifies network traffic into protocol categories.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolClass {
    Transport,
    Internet,
    Application,
    Web,
    Dns,
    Realtime,
    Unknown,
}

pub struct ProtocolClassifier;

impl ProtocolClassifier {
    pub fn classify(protocol: &str) -> ProtocolClass {
        match protocol.to_ascii_uppercase().as_str() {
            "TCP" | "UDP" | "QUIC" => {
                ProtocolClass::Transport
            }

            "IP" | "IPV4" | "IPV6" | "ICMP" | "ICMPV6" => {
                ProtocolClass::Internet
            }

            "HTTP" | "HTTPS" | "TLS" | "SSH" | "FTP" => {
                ProtocolClass::Application
            }

            "HTTP/2" | "HTTP/3" | "WEBSOCKET" => {
                ProtocolClass::Web
            }

            "DNS" | "DOH" | "DOT" | "DNSCRYPT" => {
                ProtocolClass::Dns
            }

            "WEBRTC" | "STUN" | "TURN" => {
                ProtocolClass::Realtime
            }

            _ => ProtocolClass::Unknown,
        }
    }
}
