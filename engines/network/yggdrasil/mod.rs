//! Nexus Browser Yggdrasil networking subsystem.
//!
//! Provides encrypted IPv6 mesh networking,
//! peer management, routing,
//! and identity isolation.

pub mod yggdrasil_engine;
pub mod node_manager;
pub mod peer_manager;
pub mod routing;
pub mod ipv6_manager;
pub mod identity_manager;
pub mod network_lock;


pub use yggdrasil_engine::YggdrasilEngine;
pub use node_manager::YggdrasilNodeManager;
pub use peer_manager::PeerManager;
pub use routing::RoutingManager;
pub use ipv6_manager::Ipv6Manager;
pub use identity_manager::YggdrasilIdentityManager;
pub use network_lock::YggdrasilNetworkLock;
