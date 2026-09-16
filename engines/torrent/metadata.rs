// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone)]
pub struct TorrentFile {
    pub path: String,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct TorrentMetadata {
    pub name: String,
    pub info_hash: Option<String>,
    pub piece_length: u64,
    pub total_size: u64,
    pub files: Vec<TorrentFile>,
    pub private: bool,
}

impl TorrentMetadata {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            info_hash: None,
            piece_length: 0,
            total_size: 0,
            files: Vec::new(),
            private: false,
        }
    }

    pub fn add_file(
        &mut self,
        path: impl Into<String>,
        size: u64,
    ) {
        self.total_size += size;

        self.files.push(TorrentFile {
            path: path.into(),
            size,
        });
    }

    pub fn set_info_hash(&mut self, hash: impl Into<String>) {
        self.info_hash = Some(hash.into());
    }

    pub fn set_piece_length(&mut self, length: u64) {
        self.piece_length = length;
    }

    pub fn set_private(&mut self, private: bool) {
        self.private = private;
    }

    pub fn is_multi_file(&self) -> bool {
        self.files.len() > 1
    }
}
