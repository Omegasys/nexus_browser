//! Nexus Browser IPv4 networking subsystem.
//!
//! Provides IPv4 address control,
//! interface management,
//! routing,
//! filtering,
//! and leak prevention.

pub mod ipv4_engine;
pub mod address_manager;
pub mod interface_manager;
pub mod route_manager;
pub mod packet_filter;
pub mod connection_tracker;
pub mod leak_prevention;


pub use ipv4_engine::Ipv4Engine;
pub use address_manager::Ipv4AddressManager;
pub use interface_manager::InterfaceManager;
pub use route_manager::RouteManager;
pub use packet_filter::PacketFilter;
pub use connection_tracker::ConnectionTracker;
pub use leak_prevention::Ipv4LeakPrevention;
