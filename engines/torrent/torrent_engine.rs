// SPDX-License-Identifier: GPL-3.0-or-later

use super::torrent_client::TorrentClient;
use super::torrent_security::TorrentSecurity;
use super::network_isolation::NetworkIsolation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TorrentState {
    Stopped,
    Starting,
    Running,
    Paused,
    Stopping,
    Error,
}

pub struct TorrentEngine {
    state: TorrentState,
    client: TorrentClient,
    security: TorrentSecurity,
    isolation: NetworkIsolation,
}

impl TorrentEngine {
    pub fn new() -> Self {
        Self {
            state: TorrentState::Stopped,
            client: TorrentClient::new(),
            security: TorrentSecurity::new(),
            isolation: NetworkIsolation::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if !self.security.is_safe() {
            return Err("Torrent security policy rejected startup".into());
        }

        if !self.isolation.is_ready() {
            return Err("Torrent network isolation is not ready".into());
        }

        self.state = TorrentState::Starting;

        self.client.start()?;

        self.state = TorrentState::Running;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        self.state = TorrentState::Stopping;

        self.client.stop()?;

        self.state = TorrentState::Stopped;
        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), String> {
        self.client.pause()?;
        self.state = TorrentState::Paused;
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), String> {
        self.client.resume()?;
        self.state = TorrentState::Running;
        Ok(())
    }

    pub fn state(&self) -> TorrentState {
        self.state
    }

    pub fn client(&self) -> &TorrentClient {
        &self.client
    }

    pub fn client_mut(&mut self) -> &mut TorrentClient {
        &mut self.client
    }

    pub fn security(&self) -> &TorrentSecurity {
        &self.security
    }

    pub fn security_mut(&mut self) -> &mut TorrentSecurity {
        &mut self.security
    }

    pub fn isolation(&self) -> &NetworkIsolation {
        &self.isolation
    }

    pub fn isolation_mut(&mut self) -> &mut NetworkIsolation {
        &mut self.isolation
    }
}

impl Default for TorrentEngine {
    fn default() -> Self {
        Self::new()
    }
}
