//! VPN subsystem for Nexus Browser.
//!
//! Provides VPN management,
//! encrypted tunnels,
//! routing control,
//! and kill switch protection.

pub mod vpn_engine;
pub mod vpn_manager;
pub mod tunnel;
pub mod kill_switch;


pub use vpn_engine::VpnEngine;
pub use vpn_manager::VpnManager;
pub use tunnel::VpnTunnel;
pub use kill_switch::VpnKillSwitch;
