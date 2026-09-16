//! Registry of protocols known to Nexus.

use std::collections::HashSet;

pub struct ProtocolRegistry {
    protocols: HashSet<String>,
}

impl ProtocolRegistry {
    pub fn new() -> Self {
        Self {
            protocols: HashSet::new(),
        }
    }

    pub fn register(&mut self, protocol: impl Into<String>) {
        self.protocols.insert(protocol.into());
    }

    pub fn unregister(&mut self, protocol: &str) {
        self.protocols.remove(protocol);
    }

    pub fn contains(&self, protocol: &str) -> bool {
        self.protocols.contains(protocol)
    }

    pub fn count(&self) -> usize {
        self.protocols.len()
    }

    pub fn clear(&mut self) {
        self.protocols.clear();
    }
}

impl Default for ProtocolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
