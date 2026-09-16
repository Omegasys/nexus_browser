//! Protocol detection.

#[derive(Debug, Clone)]
pub struct DetectedProtocol {
    pub protocol: String,
    pub confidence: u8,
}

pub struct ProtocolDetector;

impl ProtocolDetector {
    pub fn detect(
        protocol: &str,
    ) -> DetectedProtocol {
        DetectedProtocol {
            protocol: protocol.to_string(),
            confidence: 100,
        }
    }
}
