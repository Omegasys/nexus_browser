//! Protocol filtering event logger.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolLogAction {
    Allowed,
    Blocked,
    Detected,
}

#[derive(Debug, Clone)]
pub struct ProtocolLogEntry {
    pub protocol: String,
    pub action: ProtocolLogAction,
}

pub struct ProtocolLogger {
    entries: Vec<ProtocolLogEntry>,
}

impl ProtocolLogger {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn record(
        &mut self,
        protocol: impl Into<String>,
        action: ProtocolLogAction,
    ) {
        self.entries.push(
            ProtocolLogEntry {
                protocol: protocol.into(),
                action,
            }
        );
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for ProtocolLogger {
    fn default() -> Self {
        Self::new()
    }
}
