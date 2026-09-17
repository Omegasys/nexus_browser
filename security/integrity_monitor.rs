use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityStatus {
    Unknown,
    Trusted,
    Modified,
    Corrupted,
    Missing,
    Suspicious,
}

#[derive(Debug, Clone)]
pub struct IntegrityRecord {
    pub resource: String,
    pub expected_hash: String,
    pub observed_hash: Option<String>,
    pub status: IntegrityStatus,
    pub checked_at: SystemTime,
}

#[derive(Debug)]
pub struct IntegrityMonitor {
    records: HashMap<String, IntegrityRecord>,
}

impl IntegrityMonitor {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        resource: impl Into<String>,
        expected_hash: impl Into<String>,
    ) {
        let resource = resource.into();

        self.records.insert(
            resource.clone(),
            IntegrityRecord {
                resource,
                expected_hash: expected_hash.into(),
                observed_hash: None,
                status: IntegrityStatus::Unknown,
                checked_at: SystemTime::now(),
            },
        );
    }

    pub fn update_hash(
        &mut self,
        resource: &str,
        observed_hash: impl Into<String>,
    ) -> bool {
        let Some(record) = self.records.get_mut(resource) else {
            return false;
        };

        let observed_hash = observed_hash.into();

        record.status = if observed_hash == record.expected_hash {
            IntegrityStatus::Trusted
        } else {
            IntegrityStatus::Modified
        };

        record.observed_hash = Some(observed_hash);
        record.checked_at = SystemTime::now();

        true
    }

    pub fn mark_missing(&mut self, resource: &str) -> bool {
        let Some(record) = self.records.get_mut(resource) else {
            return false;
        };

        record.status = IntegrityStatus::Missing;
        record.checked_at = SystemTime::now();

        true
    }

    pub fn mark_corrupted(&mut self, resource: &str) -> bool {
        let Some(record) = self.records.get_mut(resource) else {
            return false;
        };

        record.status = IntegrityStatus::Corrupted;
        record.checked_at = SystemTime::now();

        true
    }

    pub fn mark_suspicious(&mut self, resource: &str) -> bool {
        let Some(record) = self.records.get_mut(resource) else {
            return false;
        };

        record.status = IntegrityStatus::Suspicious;
        record.checked_at = SystemTime::now();

        true
    }

    pub fn status(&self, resource: &str) -> Option<IntegrityStatus> {
        self.records.get(resource).map(|record| record.status)
    }

    pub fn is_trusted(&self, resource: &str) -> bool {
        self.status(resource) == Some(IntegrityStatus::Trusted)
    }

    pub fn has_problem(&self, resource: &str) -> bool {
        matches!(
            self.status(resource),
            Some(
                IntegrityStatus::Modified
                    | IntegrityStatus::Corrupted
                    | IntegrityStatus::Missing
                    | IntegrityStatus::Suspicious
            )
        )
    }

    pub fn get(&self, resource: &str) -> Option<&IntegrityRecord> {
        self.records.get(resource)
    }

    pub fn records(&self) -> impl Iterator<Item = &IntegrityRecord> {
        self.records.values()
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }
}

impl Default for IntegrityMonitor {
    fn default() -> Self {
        Self::new()
    }
}
