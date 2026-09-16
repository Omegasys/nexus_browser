//! IP protocol definitions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpProtocol {
    IPv4,
    IPv6,
    Icmp,
    Icmpv6,
}

impl IpProtocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IPv4 => "IPv4",
            Self::IPv6 => "IPv6",
            Self::Icmp => "ICMP",
            Self::Icmpv6 => "ICMPv6",
        }
    }
}
