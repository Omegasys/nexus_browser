//! Transport-layer protocol filtering.

use std::collections::HashSet;

pub struct TransportFilter {
    blocked: HashSet<String>,
}

impl TransportFilter {
    pub fn new() -> Self {
        Self {
            blocked: HashSet::new(),
        }
    }

    pub fn block(&mut self, protocol: impl Into<String>) {
        self.blocked.insert(protocol.into().to_ascii_uppercase());
    }

    pub fn unblock(&mut self, protocol: &str) {
        self.blocked.remove(&protocol.to_ascii_uppercase());
    }

    pub fn allowed(&self, protocol: &str) -> bool {
        !self.blocked.contains(
            &protocol.to_ascii_uppercase()
        )
    }

    pub fn blocked_count(&self) -> usize {
        self.blocked.len()
    }
}

impl Default for TransportFilter {
    fn default() -> Self {
        Self::new()
    }
}
