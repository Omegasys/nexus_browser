use std::net::IpAddr;
use std::time::{Duration, Instant};

/// State of a DNS query.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsQueryStatus {
    Pending,
    Resolved,
    Failed,
    Blocked,
    Cached,
}

/// A recorded DNS query.
#[derive(Debug, Clone)]
pub struct DnsQueryRecord {
    pub id: u64,
    pub hostname: String,
    pub record_type: String,
    pub resolver: Option<IpAddr>,
    pub answers: Vec<IpAddr>,
    pub status: DnsQueryStatus,
    pub duration: Duration,
    pub encrypted: bool,
    pub dnssec_validated: bool,
    pub timestamp: Instant,
}

/// DNS developer inspector.
#[derive(Debug, Clone)]
pub struct DnsInspector {
    queries: Vec<DnsQueryRecord>,
    next_id: u64,
    recording: bool,
}

/// DNS inspection result.
#[derive(Debug, Clone)]
pub struct DnsInspectionResult {
    pub total_queries: usize,
    pub successful_queries: usize,
    pub failed_queries: usize,
    pub blocked_queries: usize,
    pub encrypted_queries: usize,
    pub dnssec_validated_queries: usize,
    pub average_duration: Duration,
}

impl DnsInspector {
    pub fn new() -> Self {
        Self {
            queries: Vec::new(),
            next_id: 1,
            recording: true,
        }
    }

    pub fn start_recording(&mut self) {
        self.recording = true;
    }

    pub fn stop_recording(&mut self) {
        self.recording = false;
    }

    pub fn is_recording(&self) -> bool {
        self.recording
    }

    pub fn record_query(
        &mut self,
        hostname: impl Into<String>,
        record_type: impl Into<String>,
        resolver: Option<IpAddr>,
        answers: Vec<IpAddr>,
        status: DnsQueryStatus,
        duration: Duration,
        encrypted: bool,
        dnssec_validated: bool,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        if self.recording {
            self.queries.push(DnsQueryRecord {
                id,
                hostname: hostname.into(),
                record_type: record_type.into(),
                resolver,
                answers,
                status,
                duration,
                encrypted,
                dnssec_validated,
                timestamp: Instant::now(),
            });
        }

        id
    }

    pub fn query(&self, id: u64) -> Option<&DnsQueryRecord> {
        self.queries.iter().find(|query| query.id == id)
    }

    pub fn queries(&self) -> &[DnsQueryRecord] {
        &self.queries
    }

    pub fn inspect(&self) -> DnsInspectionResult {
        let total = self.queries.len();

        let successful = self
            .queries
            .iter()
            .filter(|query| {
                matches!(
                    query.status,
                    DnsQueryStatus::Resolved | DnsQueryStatus::Cached
                )
            })
            .count();

        let failed = self
            .queries
            .iter()
            .filter(|query| matches!(query.status, DnsQueryStatus::Failed))
            .count();

        let blocked = self
            .queries
            .iter()
            .filter(|query| matches!(query.status, DnsQueryStatus::Blocked))
            .count();

        let encrypted = self
            .queries
            .iter()
            .filter(|query| query.encrypted)
            .count();

        let dnssec = self
            .queries
            .iter()
            .filter(|query| query.dnssec_validated)
            .count();

        let total_duration: Duration =
            self.queries.iter().map(|query| query.duration).sum();

        let average_duration = if total == 0 {
            Duration::ZERO
        } else {
            total_duration / total as u32
        };

        DnsInspectionResult {
            total_queries: total,
            successful_queries: successful,
            failed_queries: failed,
            blocked_queries: blocked,
            encrypted_queries: encrypted,
            dnssec_validated_queries: dnssec,
            average_duration,
        }
    }

    pub fn clear(&mut self) {
        self.queries.clear();
    }

    pub fn query_count(&self) -> usize {
        self.queries.len()
    }
}

impl Default for DnsInspector {
    fn default() -> Self {
        Self::new()
    }
}
