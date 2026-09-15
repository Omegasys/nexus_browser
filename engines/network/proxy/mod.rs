//! Nexus Proxy Networking Engine.
//!
//! Provides generic proxy support,
//! proxy chains, authentication,
//! and routing integration.

pub mod proxy_engine;
pub mod proxy_manager;
pub mod proxy_config;
pub mod proxy_chain;
pub mod http_proxy;
pub mod https_proxy;
pub mod socks4;
pub mod socks5;
pub mod pac_manager;
pub mod authentication;
pub mod health_monitor;
pub mod failover;
pub mod leak_prevention;


pub use proxy_engine::ProxyEngine;
pub use proxy_manager::ProxyManager;
pub use proxy_config::ProxyConfig;
pub use proxy_chain::ProxyChain;
pub use authentication::ProxyAuthentication;
pub use health_monitor::HealthMonitor;
pub use failover::ProxyFailover;
pub use leak_prevention::ProxyLeakPrevention;
