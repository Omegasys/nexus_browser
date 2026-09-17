use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuarantineReason {
    IntegrityFailure,
    SuspiciousProcess,
    EngineCompromise,
    VmCompromise,
    SandboxEscape,
    PrivilegeEscalation,
    CapabilityViolation,
    MaliciousContent,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuarantineState {
    Isolated,
    Released,
    Destroyed,
}

#[derive(Debug, Clone)]
pub struct QuarantineRecord {
    pub id: u64,
    pub resource: String,
    pub reason: QuarantineReason,
    pub state: QuarantineState,
    pub created_at: SystemTime,
}

#[derive(Debug)]
pub struct QuarantineManager {
    records: HashMap<u64, QuarantineRecord>,
    next_id: u64,
}

impl QuarantineManager {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn quarantine(
        &mut self,
        resource: impl Into<String>,
        reason: QuarantineReason,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        self.records.insert(
            id,
            QuarantineRecord {
                id,
                resource: resource.into(),
                reason,
                state: QuarantineState::Isolated,
                created_at: SystemTime::now(),
            },
        );

        id
    }

    pub fn release(&mut self, id: u64) -> bool {
        let Some(record) = self.records.get_mut(&id) else {
            return false;
        };

        if record.state != QuarantineState::Isolated {
            return false;
        }

        record.state = QuarantineState::Released;
        true
    }

    pub fn destroy(&mut self, id: u64) -> bool {
        let Some(record) = self.records.get_mut(&id) else {
            return false;
        };

        record.state = QuarantineState::Destroyed;
        true
    }

    pub fn is_quarantined(&self, id: u64) -> bool {
        self.records
            .get(&id)
            .map(|record| {
                record.state == QuarantineState::Isolated
            })
            .unwrap_or(false)
    }

    pub fn get(&self, id: u64) -> Option<&QuarantineRecord> {
        self.records.get(&id)
    }

    pub fn records(&self) -> impl Iterator<Item = &QuarantineRecord> {
        self.records.values()
    }

    pub fn active(&self) -> impl Iterator<Item = &QuarantineRecord> {
        self.records.values().filter(|record| {
            record.state == QuarantineState::Isolated
        })
    }

    pub fn clear_released(&mut self) {
        self.records.retain(|_, record| {
            record.state == QuarantineState::Isolated
        });
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }
}

impl Default for QuarantineManager {
    fn default() -> Self {
        Self::new()
    }
}
