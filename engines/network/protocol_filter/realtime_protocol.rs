//! Real-time communication protocols.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealtimeProtocol {
    WebRtc,
    Stun,
    Turn,
}

impl RealtimeProtocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WebRtc => "WebRTC",
            Self::Stun => "STUN",
            Self::Turn => "TURN",
        }
    }
}
