// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceState {
    Missing,
    Requested,
    Downloading,
    Complete,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub index: u32,
    pub size: u64,
    pub state: PieceState,
    pub hash: Option<String>,
}

pub struct PieceManager {
    pieces: Vec<Piece>,
}

impl PieceManager {
    pub fn new() -> Self {
        Self {
            pieces: Vec::new(),
        }
    }

    pub fn add_piece(
        &mut self,
        index: u32,
        size: u64,
        hash: Option<String>,
    ) {
        self.pieces.push(Piece {
            index,
            size,
            state: PieceState::Missing,
            hash,
        });
    }

    pub fn request_piece(&mut self, index: u32) -> Result<(), String> {
        let piece = self
            .pieces
            .iter_mut()
            .find(|piece| piece.index == index)
            .ok_or_else(|| "Piece not found".to_string())?;

        if piece.state != PieceState::Missing {
            return Err("Piece is not available for requesting".into());
        }

        piece.state = PieceState::Requested;
        Ok(())
    }

    pub fn mark_complete(&mut self, index: u32) -> Result<(), String> {
        let piece = self
            .pieces
            .iter_mut()
            .find(|piece| piece.index == index)
            .ok_or_else(|| "Piece not found".to_string())?;

        piece.state = PieceState::Complete;
        Ok(())
    }

    pub fn mark_failed(&mut self, index: u32) {
        if let Some(piece) = self.pieces.iter_mut().find(|p| p.index == index) {
            piece.state = PieceState::Failed;
        }
    }

    pub fn next_missing(&self) -> Option<&Piece> {
        self.pieces
            .iter()
            .find(|piece| piece.state == PieceState::Missing)
    }

    pub fn completed_count(&self) -> usize {
        self.pieces
            .iter()
            .filter(|piece| piece.state == PieceState::Complete)
            .count()
    }

    pub fn total_count(&self) -> usize {
        self.pieces.len()
    }
}

impl Default for PieceManager {
    fn default() -> Self {
        Self::new()
    }
}
