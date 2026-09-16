// SPDX-License-Identifier: GPL-3.0-or-later

pub mod dns_engine;
pub mod doh;
pub mod dot;
pub mod dnscrypt;
pub mod dnssec;
pub mod resolver;
pub mod resolver_manager;
pub mod dns_cache;
pub mod dns_partitioning;
pub mod dns_leak_protection;
pub mod dns_kill_switch;

pub use dns_engine::DnsEngine;
pub use doh::DohResolver;
pub use dot::DotResolver;
pub use dnscrypt::DnsCryptResolver;
pub use dnssec::DnssecValidator;
pub use resolver::{DnsResolver, ResolverProtocol};
pub use resolver_manager::ResolverManager;
pub use dns_cache::DnsCache;
pub use dns_partitioning::DnsPartitionManager;
pub use dns_leak_protection::DnsLeakProtection;
pub use dns_kill_switch::DnsKillSwitch;
