use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeType {
    SandboxEscape,
    VmEscape,
    PrivilegeEscalation,
    CapabilityEscape,
    FilesystemEscape,
    NetworkEscape,
    ProcessEscape,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct EscapeEvent {
    pub id: u64,
    pub source: String,
    pub escape_type: EscapeType,
    pub severity: EscapeSeverity,
    pub detected_at: SystemTime,
    pub handled: bool,
}

#[derive(Debug)]
pub struct EscapeDetection {
    events: HashMap<u64, EscapeEvent>,
    next_id: u64,
}

impl EscapeDetection {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn record(
        &mut self,
        source: impl Into<String>,
        escape_type: EscapeType,
        severity: EscapeSeverity,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        self.events.insert(
            id,
            EscapeEvent {
                id,
                source: source.into(),
                escape_type,
                severity,
                detected_at: SystemTime::now(),
                handled: false,
            },
        );

        id
    }

    pub fn mark_handled(&mut self, id: u64) -> bool {
        let Some(event) = self.events.get_mut(&id) else {
            return false;
        };

        event.handled = true;
        true
    }

    pub fn is_critical(&self, id: u64) -> bool {
        self.events
            .get(&id)
            .map(|event| event.severity == EscapeSeverity::Critical)
            .unwrap_or(false)
    }

    pub fn has_unhandled_critical(&self) -> bool {
        self.events.values().any(|event| {
            event.severity == EscapeSeverity::Critical
                && !event.handled
        })
    }

    pub fn get(&self, id: u64) -> Option<&EscapeEvent> {
        self.events.get(&id)
    }

    pub fn events(&self) -> impl Iterator<Item = &EscapeEvent> {
        self.events.values()
    }

    pub fn clear_handled(&mut self) {
        self.events.retain(|_, event| !event.handled);
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl Default for EscapeDetection {
    fn default() -> Self {
        Self::new()
    }
}
