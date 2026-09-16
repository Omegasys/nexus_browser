// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientState {
    Stopped,
    Running,
    Paused,
}

#[derive(Debug, Clone)]
pub struct TorrentHandle {
    pub id: String,
    pub name: String,
    pub state: ClientState,
    pub downloaded: u64,
    pub uploaded: u64,
}

pub struct TorrentClient {
    state: ClientState,
    torrents: HashMap<String, TorrentHandle>,
}

impl TorrentClient {
    pub fn new() -> Self {
        Self {
            state: ClientState::Stopped,
            torrents: HashMap::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.state == ClientState::Running {
            return Ok(());
        }

        self.state = ClientState::Running;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        self.state = ClientState::Stopped;
        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), String> {
        if self.state != ClientState::Running {
            return Err("Torrent client is not running".into());
        }

        self.state = ClientState::Paused;
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), String> {
        if self.state != ClientState::Paused {
            return Err("Torrent client is not paused".into());
        }

        self.state = ClientState::Running;
        Ok(())
    }

    pub fn add_torrent(
        &mut self,
        id: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<(), String> {
        let id = id.into();

        if self.torrents.contains_key(&id) {
            return Err("Torrent already exists".into());
        }

        let torrent = TorrentHandle {
            id: id.clone(),
            name: name.into(),
            state: ClientState::Stopped,
            downloaded: 0,
            uploaded: 0,
        };

        self.torrents.insert(id, torrent);
        Ok(())
    }

    pub fn remove_torrent(&mut self, id: &str) -> Result<(), String> {
        self.torrents
            .remove(id)
            .map(|_| ())
            .ok_or_else(|| "Torrent not found".into())
    }

    pub fn torrent(&self, id: &str) -> Option<&TorrentHandle> {
        self.torrents.get(id)
    }

    pub fn torrents(&self) -> impl Iterator<Item = &TorrentHandle> {
        self.torrents.values()
    }

    pub fn state(&self) -> ClientState {
        self.state
    }
}

impl Default for TorrentClient {
    fn default() -> Self {
        Self::new()
    }
}
