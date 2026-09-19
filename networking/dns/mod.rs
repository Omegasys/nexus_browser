pub mod dns_manager;
pub mod secure_dns;
pub mod doh;
pub mod dot;
pub mod dnscrypt;
pub mod dnssec;

pub use dns_manager::DnsManager;
pub use secure_dns::{DnsMode, SecureDnsConfig};
pub use doh::DohResolver;
pub use dot::DotResolver;
pub use dnscrypt::DnsCryptResolver;
pub use dnssec::{DnssecMode, DnssecValidator};
