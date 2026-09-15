//! Nexus Browser Tor networking subsystem.
//!
//! Provides Tor routing,
//! circuit management,
//! bridge support,
//! onion services,
//! and Tor-specific network controls.

pub mod tor_engine;
pub mod tor_controller;
pub mod circuit_manager;
pub mod bridge_manager;
pub mod onion_service;
pub mod identity_manager;
pub mod network_lock;


pub use tor_engine::TorEngine;
pub use tor_controller::TorController;
pub use circuit_manager::CircuitManager;
pub use bridge_manager::BridgeManager;
pub use onion_service::OnionServiceManager;
pub use identity_manager::TorIdentityManager;
pub use network_lock::TorNetworkLock;
