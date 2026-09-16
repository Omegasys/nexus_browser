//! Common protocol and port associations.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortProtocol {
    pub port: u16,
    pub tcp: bool,
    pub udp: bool,
}

impl PortProtocol {
    pub fn new(
        port: u16,
        tcp: bool,
        udp: bool,
    ) -> Self {
        Self {
            port,
            tcp,
            udp,
        }
    }

    pub fn tcp(port: u16) -> Self {
        Self::new(port, true, false)
    }

    pub fn udp(port: u16) -> Self {
        Self::new(port, false, true)
    }

    pub fn both(port: u16) -> Self {
        Self::new(port, true, true)
    }
}
