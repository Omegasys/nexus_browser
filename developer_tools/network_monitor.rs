use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};

/// Protocol used by a network request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequestProtocol {
    Http,
    Https,
    Http2,
    Http3,
    WebSocket,
    WebRtc,
    Dns,
    DoH,
    DoT,
    Tcp,
    Udp,
    Quic,
    Unknown,
}

/// Current connection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connecting,
    Connected,
    Closing,
    Closed,
    Failed,
    Blocked,
}

/// A monitored network connection.
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub id: u64,
    pub local_address: Option<SocketAddr>,
    pub remote_address: Option<SocketAddr>,
    pub protocol: RequestProtocol,
    pub state: ConnectionState,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub created_at: Instant,
    pub last_activity: Instant,
}

/// A monitored network request.
#[derive(Debug, Clone)]
pub struct NetworkRequest {
    pub id: u64,
    pub url: String,
    pub method: String,
    pub protocol: RequestProtocol,
    pub status_code: Option<u16>,
    pub request_bytes: u64,
    pub response_bytes: u64,
    pub duration: Duration,
    pub blocked: bool,
    pub failure_reason: Option<String>,
}

/// Developer network monitor.
#[derive(Debug, Clone)]
pub struct NetworkMonitor {
    connections: HashMap<u64, ConnectionInfo>,
    requests: HashMap<u64, NetworkRequest>,
    next_connection_id: u64,
    next_request_id: u64,
    recording: bool,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            requests: HashMap::new(),
            next_connection_id: 1,
            next_request_id: 1,
            recording: true,
        }
    }

    pub fn start_recording(&mut self) {
        self.recording = true;
    }

    pub fn stop_recording(&mut self) {
        self.recording = false;
    }

    pub fn is_recording(&self) -> bool {
        self.recording
    }

    pub fn add_connection(
        &mut self,
        local_address: Option<SocketAddr>,
        remote_address: Option<SocketAddr>,
        protocol: RequestProtocol,
    ) -> u64 {
        let id = self.next_connection_id;
        self.next_connection_id += 1;

        let now = Instant::now();

        self.connections.insert(
            id,
            ConnectionInfo {
                id,
                local_address,
                remote_address,
                protocol,
                state: ConnectionState::Connecting,
                bytes_sent: 0,
                bytes_received: 0,
                created_at: now,
                last_activity: now,
            },
        );

        id
    }

    pub fn update_connection_state(
        &mut self,
        id: u64,
        state: ConnectionState,
    ) -> bool {
        if let Some(connection) = self.connections.get_mut(&id) {
            connection.state = state;
            connection.last_activity = Instant::now();
            true
        } else {
            false
        }
    }

    pub fn record_bytes(
        &mut self,
        id: u64,
        sent: u64,
        received: u64,
    ) -> bool {
        if let Some(connection) = self.connections.get_mut(&id) {
            connection.bytes_sent += sent;
            connection.bytes_received += received;
            connection.last_activity = Instant::now();
            true
        } else {
            false
        }
    }

    pub fn add_request(
        &mut self,
        url: impl Into<String>,
        method: impl Into<String>,
        protocol: RequestProtocol,
    ) -> u64 {
        let id = self.next_request_id;
        self.next_request_id += 1;

        self.requests.insert(
            id,
            NetworkRequest {
                id,
                url: url.into(),
                method: method.into(),
                protocol,
                status_code: None,
                request_bytes: 0,
                response_bytes: 0,
                duration: Duration::ZERO,
                blocked: false,
                failure_reason: None,
            },
        );

        id
    }

    pub fn complete_request(
        &mut self,
        id: u64,
        status_code: Option<u16>,
        request_bytes: u64,
        response_bytes: u64,
        duration: Duration,
    ) -> bool {
        if let Some(request) = self.requests.get_mut(&id) {
            request.status_code = status_code;
            request.request_bytes = request_bytes;
            request.response_bytes = response_bytes;
            request.duration = duration;
            true
        } else {
            false
        }
    }

    pub fn block_request(
        &mut self,
        id: u64,
        reason: impl Into<String>,
    ) -> bool {
        if let Some(request) = self.requests.get_mut(&id) {
            request.blocked = true;
            request.failure_reason = Some(reason.into());
            true
        } else {
            false
        }
    }

    pub fn connections(&self) -> impl Iterator<Item = &ConnectionInfo> {
        self.connections.values()
    }

    pub fn requests(&self) -> impl Iterator<Item = &NetworkRequest> {
        self.requests.values()
    }

    pub fn connection(&self, id: u64) -> Option<&ConnectionInfo> {
        self.connections.get(&id)
    }

    pub fn request(&self, id: u64) -> Option<&NetworkRequest> {
        self.requests.get(&id)
    }

    pub fn clear(&mut self) {
        self.connections.clear();
        self.requests.clear();
    }

    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    pub fn request_count(&self) -> usize {
        self.requests.len()
    }

    pub fn total_bytes_sent(&self) -> u64 {
        self.connections
            .values()
            .map(|connection| connection.bytes_sent)
            .sum()
    }

    pub fn total_bytes_received(&self) -> u64 {
        self.connections
            .values()
            .map(|connection| connection.bytes_received)
            .sum()
    }

    pub fn is_local_address(address: IpAddr) -> bool {
        match address {
            IpAddr::V4(ip) => {
                ip.is_private()
                    || ip.is_loopback()
                    || ip.is_link_local()
            }
            IpAddr::V6(ip) => {
                ip.is_loopback()
                    || ip.is_unique_local()
                    || ip.is_unicast_link_local()
            }
        }
    }
}

impl Default for NetworkMonitor {
    fn default() -> Self {
        Self::new()
    }
}
