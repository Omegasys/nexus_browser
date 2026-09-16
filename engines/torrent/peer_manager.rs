// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerState {
    Discovered,
    Connecting,
    Connected,
    Choked,
    Disconnected,
    Blocked,
}

#[derive(Debug, Clone)]
pub struct Peer {
    pub id: String,
    pub address: String,
    pub port: u16,
    pub state: PeerState,
    pub downloaded: u64,
    pub uploaded: u64,
}

pub struct PeerManager {
    peers: HashMap<String, Peer>,
}

impl PeerManager {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    pub fn add_peer(
        &mut self,
        id: impl Into<String>,
        address: impl Into<String>,
        port: u16,
    ) {
        let id = id.into();

        self.peers.insert(
            id.clone(),
            Peer {
                id,
                address: address.into(),
                port,
                state: PeerState::Discovered,
                downloaded: 0,
                uploaded: 0,
            },
        );
    }

    pub fn remove_peer(&mut self, id: &str) {
        self.peers.remove(id);
    }

    pub fn block_peer(&mut self, id: &str) {
        if let Some(peer) = self.peers.get_mut(id) {
            peer.state = PeerState::Blocked;
        }
    }

    pub fn connected_peers(&self) -> impl Iterator<Item = &Peer> {
        self.peers
            .values()
            .filter(|peer| peer.state == PeerState::Connected)
    }

    pub fn peers(&self) -> impl Iterator<Item = &Peer> {
        self.peers.values()
    }
}

impl Default for PeerManager {
    fn default() -> Self {
        Self::new()
    }
}
