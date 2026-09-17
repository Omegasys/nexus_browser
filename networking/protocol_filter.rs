use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    Tcp,
    Udp,
    Quic,
    Ipv4,
    Ipv6,
    Icmp,
    Icmpv6,
    Http,
    Https,
    Http2,
    Http3,
    WebSocket,
    WebRtc,
    Dns,
    Doh,
    Dot,
    DnsCrypt,
    Ssh,
    Ftp,
    Sftp,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolFilterAction {
    Allow,
    Block,
    Log,
}

#[derive(Debug, Clone)]
pub struct ProtocolFilter {
    default_action: ProtocolFilterAction,
    rules: HashSet<Protocol>,
}

impl ProtocolFilter {
    pub fn new() -> Self {
        Self {
            default_action: ProtocolFilterAction::Allow,
            rules: HashSet::new(),
        }
    }

    pub fn set_default_action(&mut self, action: ProtocolFilterAction) {
        self.default_action = action;
    }

    pub fn allow(&mut self, protocol: Protocol) {
        self.rules.remove(&protocol);
    }

    pub fn block(&mut self, protocol: Protocol) {
        self.rules.insert(protocol);
    }

    pub fn is_blocked(&self, protocol: Protocol) -> bool {
        match self.default_action {
            ProtocolFilterAction::Block => !self.rules.contains(&protocol),
            ProtocolFilterAction::Allow => self.rules.contains(&protocol),
            ProtocolFilterAction::Log => false,
        }
    }

    pub fn action_for(&self, protocol: Protocol) -> ProtocolFilterAction {
        if self.is_blocked(protocol) {
            ProtocolFilterAction::Block
        } else if self.rules.contains(&protocol) {
            ProtocolFilterAction::Allow
        } else {
            self.default_action
        }
    }

    pub fn clear(&mut self) {
        self.rules.clear();
    }

    pub fn blocked_protocols(&self) -> impl Iterator<Item = &Protocol> {
        self.rules.iter()
    }
}

impl Default for ProtocolFilter {
    fn default() -> Self {
        Self::new()
    }
}
