//! Web protocol filtering.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebProtocol {
    Http,
    Https,
    Http2,
    Http3,
    WebSocket,
}

impl WebProtocol {
    pub fn secure(&self) -> bool {
        matches!(
            self,
            Self::Https |
            Self::Http2 |
            Self::Http3
        )
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Http => "HTTP",
            Self::Https => "HTTPS",
            Self::Http2 => "HTTP/2",
            Self::Http3 => "HTTP/3",
            Self::WebSocket => "WebSocket",
        }
    }
}
