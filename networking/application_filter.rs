use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplicationProtocol {
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
    Ftp,
    Sftp,
    Ssh,
    Torrent,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationAction {
    Allow,
    Block,
    Log,
}

#[derive(Debug, Clone)]
pub struct ApplicationRule {
    pub protocol: ApplicationProtocol,
    pub action: ApplicationAction,
}

impl ApplicationRule {
    pub fn new(
        protocol: ApplicationProtocol,
        action: ApplicationAction,
    ) -> Self {
        Self { protocol, action }
    }
}

#[derive(Debug)]
pub struct ApplicationFilter {
    rules: HashMap<ApplicationProtocol, ApplicationAction>,
    default_action: ApplicationAction,
}

impl Default for ApplicationFilter {
    fn default() -> Self {
        Self {
            rules: HashMap::new(),
            default_action: ApplicationAction::Allow,
        }
    }
}

impl ApplicationFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_rule(
        &mut self,
        protocol: ApplicationProtocol,
        action: ApplicationAction,
    ) {
        self.rules.insert(protocol, action);
    }

    pub fn remove_rule(&mut self, protocol: ApplicationProtocol) {
        self.rules.remove(&protocol);
    }

    pub fn set_default_action(&mut self, action: ApplicationAction) {
        self.default_action = action;
    }

    pub fn evaluate(&self, protocol: ApplicationProtocol) -> ApplicationAction {
        self.rules
            .get(&protocol)
            .copied()
            .unwrap_or(self.default_action)
    }

    pub fn allows(&self, protocol: ApplicationProtocol) -> bool {
        self.evaluate(protocol) == ApplicationAction::Allow
    }

    pub fn rules(&self) -> &HashMap<ApplicationProtocol, ApplicationAction> {
        &self.rules
    }
}
