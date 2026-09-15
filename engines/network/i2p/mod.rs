//! Nexus Browser I2P networking subsystem.
//!
//! Provides I2P routing,
//! tunnels, destinations,
//! and network leak protection.

pub mod i2p_engine;
pub mod router;
pub mod tunnel_manager;
pub mod destination_manager;
pub mod network_lock;


pub use i2p_engine::I2pEngine;
pub use router::I2pRouter;
pub use tunnel_manager::TunnelManager;
pub use destination_manager::DestinationManager;
pub use network_lock::I2pNetworkLock;
