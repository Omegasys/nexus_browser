//! High-level protocol policy.

#[derive(Debug, Clone)]
pub struct ProtocolPolicy {
    pub allow_tcp: bool,
    pub allow_udp: bool,
    pub allow_quic: bool,
    pub allow_ipv4: bool,
    pub allow_ipv6: bool,
    pub allow_http: bool,
    pub allow_https: bool,
    pub allow_websocket: bool,
    pub allow_webrtc: bool,
    pub allow_plain_dns: bool,
}

impl ProtocolPolicy {
    pub fn lockdown() -> Self {
        Self {
            allow_tcp: false,
            allow_udp: false,
            allow_quic: false,
            allow_ipv4: false,
            allow_ipv6: false,
            allow_http: false,
            allow_https: false,
            allow_websocket: false,
            allow_webrtc: false,
            allow_plain_dns: false,
        }
    }
}

impl Default for ProtocolPolicy {
    fn default() -> Self {
        Self {
            allow_tcp: true,
            allow_udp: true,
            allow_quic: true,
            allow_ipv4: true,
            allow_ipv6: true,
            allow_http: false,
            allow_https: true,
            allow_websocket: true,
            allow_webrtc: true,
            allow_plain_dns: false,
        }
    }
}
