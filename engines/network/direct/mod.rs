//! Nexus Direct Networking Engine.
//!
//! Provides standard internet connectivity
//! without additional routing layers.

pub mod direct_engine;
pub mod connection_manager;
pub mod interface_selector;
pub mod route_manager;
pub mod protocol_handler;
pub mod socket_manager;
pub mod direct_policy;
pub mod leak_prevention;


pub use direct_engine::DirectEngine;
pub use connection_manager::ConnectionManager;
pub use interface_selector::InterfaceSelector;
pub use route_manager::RouteManager;
pub use protocol_handler::ProtocolHandler;
pub use socket_manager::SocketManager;
pub use direct_policy::DirectPolicy;
pub use leak_prevention::DirectLeakPrevention;
