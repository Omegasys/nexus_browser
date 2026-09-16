// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    KeepAlive,
    Choke,
    Unchoke,
    Interested,
    NotInterested,
    Have,
    Bitfield,
    Request,
    Piece,
    Cancel,
    Port,
    Extended,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct PeerMessage {
    pub message_type: MessageType,
    pub payload: Vec<u8>,
}

pub struct BitTorrentProtocol;

impl BitTorrentProtocol {
    pub const PROTOCOL_NAME: &'static str = "BitTorrent protocol";

    pub fn handshake(info_hash: &[u8], peer_id: &[u8]) -> Vec<u8> {
        let mut data = Vec::with_capacity(68);

        data.push(19);
        data.extend_from_slice(Self::PROTOCOL_NAME.as_bytes());
        data.extend_from_slice(&[0u8; 8]);

        let mut hash = [0u8; 20];
        let hash_len = info_hash.len().min(hash.len());
        hash[..hash_len].copy_from_slice(&info_hash[..hash_len]);

        let mut id = [0u8; 20];
        let id_len = peer_id.len().min(id.len());
        id[..id_len].copy_from_slice(&peer_id[..id_len]);

        data.extend_from_slice(&hash);
        data.extend_from_slice(&id);

        data
    }

    pub fn keep_alive() -> PeerMessage {
        PeerMessage {
            message_type: MessageType::KeepAlive,
            payload: Vec::new(),
        }
    }

    pub fn interested() -> PeerMessage {
        PeerMessage {
            message_type: MessageType::Interested,
            payload: Vec::new(),
        }
    }

    pub fn request(
        piece_index: u32,
        begin: u32,
        length: u32,
    ) -> PeerMessage {
        let mut payload = Vec::with_capacity(12);

        payload.extend_from_slice(&piece_index.to_be_bytes());
        payload.extend_from_slice(&begin.to_be_bytes());
        payload.extend_from_slice(&length.to_be_bytes());

        PeerMessage {
            message_type: MessageType::Request,
            payload,
        }
    }
}
