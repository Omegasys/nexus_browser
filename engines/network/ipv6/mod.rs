//! Nexus Browser IPv6 networking subsystem.
//!
//! Provides IPv6 address control,
//! routing,
//! filtering,
//! privacy addressing,
//! and leak prevention.

pub mod ipv6_engine;
pub mod address_manager;
pub mod interface_manager;
pub mod route_manager;
pub mod neighbor_manager;
pub mod packet_filter;
pub mod privacy_address;
pub mod connection_tracker;
pub mod leak_prevention;


pub use ipv6_engine::Ipv6Engine;
pub use address_manager::Ipv6AddressManager;
pub use interface_manager::InterfaceManager;
pub use route_manager::RouteManager;
pub use neighbor_manager::NeighborManager;
pub use packet_filter::PacketFilter;
pub use privacy_address::PrivacyAddressManager;
pub use connection_tracker::ConnectionTracker;
pub use leak_prevention::Ipv6LeakPrevention;
