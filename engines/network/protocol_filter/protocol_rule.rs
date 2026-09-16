//! Individual protocol filtering rules.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolAction {
    Allow,
    Block,
    Log,
}

#[derive(Debug, Clone)]
pub struct ProtocolRule {
    pub protocol: String,
    pub action: ProtocolAction,
    pub enabled: bool,
}

impl ProtocolRule {
    pub fn new(
        protocol: impl Into<String>,
        action: ProtocolAction,
    ) -> Self {
        Self {
            protocol: protocol.into(),
            action,
            enabled: true,
        }
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }
}
