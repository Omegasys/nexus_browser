use std::collections::HashMap;

/// Action taken by a network policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyAction {
    Allow,
    Block,
    Redirect,
    Isolate,
    Log,
}

/// A network policy rule.
#[derive(Debug, Clone)]
pub struct PolicyRule {
    pub id: String,
    pub name: String,
    pub action: PolicyAction,
    pub protocol: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub enabled: bool,
    pub priority: i32,
}

/// Result of inspecting a network policy.
#[derive(Debug, Clone)]
pub struct PolicyInspectionResult {
    pub matched_rule: Option<String>,
    pub action: Option<PolicyAction>,
    pub allowed: bool,
    pub isolated: bool,
    pub logged: bool,
}

/// Developer-facing network policy inspector.
#[derive(Debug, Clone)]
pub struct NetworkPolicyInspector {
    rules: HashMap<String, PolicyRule>,
}

impl NetworkPolicyInspector {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, rule: PolicyRule) {
        self.rules.insert(rule.id.clone(), rule);
    }

    pub fn remove_rule(&mut self, id: &str) -> Option<PolicyRule> {
        self.rules.remove(id)
    }

    pub fn get_rule(&self, id: &str) -> Option<&PolicyRule> {
        self.rules.get(id)
    }

    pub fn rules(&self) -> impl Iterator<Item = &PolicyRule> {
        self.rules.values()
    }

    pub fn inspect(
        &self,
        protocol: Option<&str>,
        host: Option<&str>,
        port: Option<u16>,
    ) -> PolicyInspectionResult {
        let mut matching: Vec<&PolicyRule> = self
            .rules
            .values()
            .filter(|rule| rule.enabled)
            .filter(|rule| {
                rule.protocol
                    .as_deref()
                    .map(|value| Some(value) == protocol)
                    .unwrap_or(true)
            })
            .filter(|rule| {
                rule.host
                    .as_deref()
                    .map(|value| Some(value) == host)
                    .unwrap_or(true)
            })
            .filter(|rule| {
                rule.port
                    .map(|value| Some(value) == port)
                    .unwrap_or(true)
            })
            .collect();

        matching.sort_by(|a, b| b.priority.cmp(&a.priority));

        let rule = matching.first().copied();

        match rule {
            Some(rule) => PolicyInspectionResult {
                matched_rule: Some(rule.id.clone()),
                action: Some(rule.action),
                allowed: matches!(
                    rule.action,
                    PolicyAction::Allow | PolicyAction::Log
                ),
                isolated: matches!(rule.action, PolicyAction::Isolate),
                logged: matches!(
                    rule.action,
                    PolicyAction::Log
                        | PolicyAction::Allow
                        | PolicyAction::Block
                        | PolicyAction::Isolate
                ),
            },
            None => PolicyInspectionResult {
                matched_rule: None,
                action: None,
                allowed: true,
                isolated: false,
                logged: false,
            },
        }
    }

    pub fn enable_rule(&mut self, id: &str) -> bool {
        if let Some(rule) = self.rules.get_mut(id) {
            rule.enabled = true;
            true
        } else {
            false
        }
    }

    pub fn disable_rule(&mut self, id: &str) -> bool {
        if let Some(rule) = self.rules.get_mut(id) {
            rule.enabled = false;
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.rules.clear();
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl Default for NetworkPolicyInspector {
    fn default() -> Self {
        Self::new()
    }
}
