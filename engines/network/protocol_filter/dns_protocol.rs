//! DNS protocol filtering.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsProtocol {
    System,
    PlainDns,
    Doh,
    Dot,
    DnsCrypt,
    TorDns,
}

impl DnsProtocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::PlainDns => "dns",
            Self::Doh => "doh",
            Self::Dot => "dot",
            Self::DnsCrypt => "dnscrypt",
            Self::TorDns => "tor-dns",
        }
    }
}
