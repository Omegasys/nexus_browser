//! Nexus Browser Freenet networking subsystem.
//!
//! Provides decentralized storage,
//! peer networking, requests,
//! and privacy-focused publishing.

pub mod freenet_engine;
pub mod node_manager;
pub mod peer_manager;
pub mod data_store;
pub mod request_manager;
pub mod identity_manager;
pub mod network_lock;


pub use freenet_engine::FreenetEngine;
pub use node_manager::FreenetNodeManager;
pub use peer_manager::PeerManager;
pub use data_store::DataStore;
pub use request_manager::RequestManager;
pub use identity_manager::FreenetIdentityManager;
pub use network_lock::FreenetNetworkLock;
