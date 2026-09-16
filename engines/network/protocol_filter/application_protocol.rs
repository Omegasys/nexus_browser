//! Application protocol definitions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationProtocol {
    Http,
    Https,
    Tls,
    Ssh,
    Ftp,
    Sftp,
    WebSocket,
    Unknown,
}

impl ApplicationProtocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Http => "HTTP",
            Self::Https => "HTTPS",
            Self::Tls => "TLS",
            Self::Ssh => "SSH",
            Self::Ftp => "FTP",
            Self::Sftp => "SFTP",
            Self::WebSocket => "WebSocket",
            Self::Unknown => "Unknown",
        }
    }
}
