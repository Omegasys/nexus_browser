use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BounceTrackingMode {
    Disabled,
    Detect,
    Block,
}

#[derive(Debug, Clone)]
pub struct BounceRecord {
    pub domain: String,
    pub first_seen: Instant,
    pub last_seen: Instant,
    pub redirect_count: u32,
}

#[derive(Debug, Clone)]
pub struct BounceTrackingProtection {
    pub mode: BounceTrackingMode,
    pub threshold: u32,
    pub observation_window: Duration,
    records: HashMap<String, BounceRecord>,
}

impl BounceTrackingProtection {
    pub fn new() -> Self {
        Self {
            mode: BounceTrackingMode::Block,
            threshold: 2,
            observation_window: Duration::from_secs(300),
            records: HashMap::new(),
        }
    }

    pub fn set_mode(&mut self, mode: BounceTrackingMode) {
        self.mode = mode;
    }

    pub fn observe_redirect(&mut self, domain: &str) -> bool {
        if self.mode == BounceTrackingMode::Disabled {
            return false;
        }

        let now = Instant::now();

        let record = self
            .records
            .entry(domain.to_ascii_lowercase())
            .or_insert(BounceRecord {
                domain: domain.to_ascii_lowercase(),
                first_seen: now,
                last_seen: now,
                redirect_count: 0,
            });

        if now.duration_since(record.first_seen) > self.observation_window {
            record.first_seen = now;
            record.redirect_count = 0;
        }

        record.last_seen = now;
        record.redirect_count += 1;

        self.mode == BounceTrackingMode::Block
            && record.redirect_count >= self.threshold
    }

    pub fn is_suspected_bounce(&self, domain: &str) -> bool {
        let Some(record) = self.records.get(&domain.to_ascii_lowercase()) else {
            return false;
        };

        record.redirect_count >= self.threshold
            && record.last_seen.duration_since(record.first_seen)
                <= self.observation_window
    }

    pub fn clear_expired(&mut self) {
        let window = self.observation_window;

        self.records
            .retain(|_, record| record.last_seen.elapsed() <= window);
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }
}

impl Default for BounceTrackingProtection {
    fn default() -> Self {
        Self::new()
    }
}
