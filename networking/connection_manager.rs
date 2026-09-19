use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};

use super::application_filter::{ApplicationFilter, ApplicationProtocol};
use super::ip_filter::IpFilter;
use super::transport_filter::{TransportFilter, TransportProtocol};

pub type ConnectionId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    New,
    Connecting,
    Established,
    Closing,
    Closed,
    Blocked,
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub id: ConnectionId,
    pub local: Option<SocketAddr>,
    pub remote: SocketAddr,
    pub transport: TransportProtocol,
    pub application: ApplicationProtocol,
    pub state: ConnectionState,
    pub created_at: Instant,
    pub last_activity: Instant,
}

impl Connection {
    pub fn new(
        id: ConnectionId,
        remote: SocketAddr,
        transport: TransportProtocol,
        application: ApplicationProtocol,
    ) -> Self {
        let now = Instant::now();

        Self {
            id,
            local: None,
            remote,
            transport,
            application,
            state: ConnectionState::New,
            created_at: now,
            last_activity: now,
        }
    }

    pub fn touch(&mut self) {
        self.last_activity = Instant::now();
    }

    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }
}

#[derive(Debug)]
pub struct ConnectionManager {
    connections: HashMap<ConnectionId, Connection>,
    next_id: ConnectionId,
    ip_filter: IpFilter,
    transport_filter: TransportFilter,
    application_filter: ApplicationFilter,
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            next_id: 1,
            ip_filter: IpFilter::new(),
            transport_filter: TransportFilter::new(),
            application_filter: ApplicationFilter::new(),
        }
    }

    pub fn create_connection(
        &mut self,
        remote: SocketAddr,
        transport: TransportProtocol,
        application: ApplicationProtocol,
    ) -> Option<ConnectionId> {
        if !self.ip_filter.allows(remote.ip()) {
            return None;
        }

        if !self.transport_filter.allows(transport) {
            return None;
        }

        if !self.application_filter.allows(application) {
            return None;
        }

        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);

        let connection =
            Connection::new(id, remote, transport, application);

        self.connections.insert(id, connection);

        Some(id)
    }

    pub fn set_state(
        &mut self,
        id: ConnectionId,
        state: ConnectionState,
    ) -> bool {
        if let Some(connection) = self.connections.get_mut(&id) {
            connection.state = state;
            connection.touch();
            true
        } else {
            false
        }
    }

    pub fn touch(&mut self, id: ConnectionId) -> bool {
        if let Some(connection) = self.connections.get_mut(&id) {
            connection.touch();
            true
        } else {
            false
        }
    }

    pub fn close(&mut self, id: ConnectionId) -> bool {
        self.set_state(id, ConnectionState::Closed)
    }

    pub fn remove_closed(&mut self) {
        self.connections
            .retain(|_, connection| connection.state != ConnectionState::Closed);
    }

    pub fn connection(&self, id: ConnectionId) -> Option<&Connection> {
        self.connections.get(&id)
    }

    pub fn connections(&self) -> &HashMap<ConnectionId, Connection> {
        &self.connections
    }

    pub fn ip_filter_mut(&mut self) -> &mut IpFilter {
        &mut self.ip_filter
    }

    pub fn transport_filter_mut(&mut self) -> &mut TransportFilter {
        &mut self.transport_filter
    }

    pub fn application_filter_mut(&mut self) -> &mut ApplicationFilter {
        &mut self.application_filter
    }

    pub fn active_count(&self) -> usize {
        self.connections
            .values()
            .filter(|connection| {
                matches!(
                    connection.state,
                    ConnectionState::Connecting
                        | ConnectionState::Established
                )
            })
            .count()
    }

    pub fn has_remote_address(&self, address: IpAddr) -> bool {
        self.connections
            .values()
            .any(|connection| connection.remote.ip() == address)
    }
}
