//! Nexus Browser GNUnet networking subsystem.
//!
//! Provides decentralized peer networking,
//! services, routing, storage,
//! and identity separation.

pub mod gnunet_engine;
pub mod service_manager;
pub mod peer_manager;
pub mod routing;
pub mod datastore;
pub mod identity_manager;
pub mod network_lock;


pub use gnunet_engine::GnUnetEngine;
pub use service_manager::ServiceManager;
pub use peer_manager::PeerManager;
pub use routing::RoutingManager;
pub use datastore::DataStore;
pub use identity_manager::GnUnetIdentityManager;
pub use network_lock::GnUnetNetworkLock;
