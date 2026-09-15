//! Nexus Browser IPFS networking subsystem.
//!
//! Provides decentralized content networking,
//! peer discovery, gateways, and content storage.

pub mod ipfs_engine;
pub mod node_manager;
pub mod peer_manager;
pub mod content_manager;
pub mod gateway_manager;
pub mod pin_manager;
pub mod network_lock;


pub use ipfs_engine::IpfsEngine;
pub use node_manager::IpfsNodeManager;
pub use peer_manager::PeerManager;
pub use content_manager::ContentManager;
pub use gateway_manager::GatewayManager;
pub use pin_manager::PinManager;
pub use network_lock::IpfsNetworkLock;
