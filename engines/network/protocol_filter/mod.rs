//! Nexus protocol filtering subsystem.
//!
//! Provides protocol classification, policy enforcement,
//! transport filtering, protocol detection, logging,
//! and emergency protocol locking.

pub mod protocol_filter;
pub mod protocol_registry;
pub mod protocol_rule;
pub mod protocol_classifier;
pub mod transport_filter;
pub mod application_protocol;
pub mod ip_protocol;
pub mod port_protocol;
pub mod dns_protocol;
pub mod web_protocol;
pub mod realtime_protocol;
pub mod protocol_policy;
pub mod protocol_detector;
pub mod protocol_logger;
pub mod protocol_lock;

pub use protocol_filter::ProtocolFilter;
pub use protocol_registry::ProtocolRegistry;
pub use protocol_rule::{ProtocolAction, ProtocolRule};
pub use protocol_classifier::ProtocolClassifier;
pub use transport_filter::TransportFilter;
pub use application_protocol::ApplicationProtocol;
pub use ip_protocol::IpProtocol;
pub use port_protocol::PortProtocol;
pub use dns_protocol::DnsProtocol;
pub use web_protocol::WebProtocol;
pub use realtime_protocol::RealtimeProtocol;
pub use protocol_policy::ProtocolPolicy;
pub use protocol_detector::ProtocolDetector;
pub use protocol_logger::ProtocolLogger;
pub use protocol_lock::ProtocolLock;
