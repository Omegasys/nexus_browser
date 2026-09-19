use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnssecMode {
    Disabled,
    Validate,
    RequireValid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnssecStatus {
    Unknown,
    Secure,
    Insecure,
    Bogus,
    Indeterminate,
}

#[derive(Debug, Clone)]
pub struct DnssecResult {
    pub hostname: String,
    pub status: DnssecStatus,
    pub validated_at: Instant,
}

impl DnssecResult {
    pub fn new(hostname: impl Into<String>, status: DnssecStatus) -> Self {
        Self {
            hostname: hostname.into(),
            status,
            validated_at: Instant::now(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.status == DnssecStatus::Secure
    }

    pub fn age(&self) -> Duration {
        self.validated_at.elapsed()
    }
}

#[derive(Debug)]
pub struct DnssecValidator {
    mode: DnssecMode,
    results: HashMap<String, DnssecResult>,
    cache_duration: Duration,
}

impl Default for DnssecValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl DnssecValidator {
    pub fn new() -> Self {
        Self {
            mode: DnssecMode::Validate,
            results: HashMap::new(),
            cache_duration: Duration::from_secs(300),
        }
    }

    pub fn set_mode(&mut self, mode: DnssecMode) {
        self.mode = mode;
    }

    pub fn mode(&self) -> DnssecMode {
        self.mode
    }

    pub fn set_cache_duration(&mut self, duration: Duration) {
        self.cache_duration = duration;
    }

    pub fn record_result(
        &mut self,
        hostname: impl Into<String>,
        status: DnssecStatus,
    ) {
        let hostname = hostname.into();

        let result = DnssecResult::new(hostname.clone(), status);

        self.results.insert(hostname, result);
    }

    pub fn result(&self, hostname: &str) -> Option<&DnssecResult> {
        self.results.get(hostname).filter(|result| {
            result.age() <= self.cache_duration
        })
    }

    pub fn validate(&self, hostname: &str) -> Result<bool, String> {
        match self.mode {
            DnssecMode::Disabled => Ok(true),

            DnssecMode::Validate => {
                match self.result(hostname) {
                    Some(result) => Ok(result.is_valid()),
                    None => Err(
                        "DNSSEC validation result is unavailable".into()
                    ),
                }
            }

            DnssecMode::RequireValid => {
                match self.result(hostname) {
                    Some(result) if result.is_valid() => Ok(true),
                    Some(_) => Ok(false),
                    None => Err(
                        "DNSSEC validation is required but no valid result exists"
                            .into(),
                    ),
                }
            }
        }
    }

    pub fn remove_expired(&mut self) {
        let duration = self.cache_duration;

        self.results.retain(|_, result| {
            result.age() <= duration
        });
    }

    pub fn clear(&mut self) {
        self.results.clear();
    }

    pub fn results(&self) -> &HashMap<String, DnssecResult> {
        &self.results
    }
}
