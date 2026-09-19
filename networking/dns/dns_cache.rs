use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct DnsCacheEntry {
    pub hostname: String,
    pub addresses: Vec<IpAddr>,
    pub expires_at: Instant,
    pub secure: bool,
    pub validated: bool,
}

impl DnsCacheEntry {
    pub fn new(
        hostname: impl Into<String>,
        addresses: Vec<IpAddr>,
        ttl: Duration,
        secure: bool,
        validated: bool,
    ) -> Self {
        Self {
            hostname: hostname.into(),
            addresses,
            expires_at: Instant::now() + ttl,
            secure,
            validated,
        }
    }

    pub fn expired(&self) -> bool {
        Instant::now() >= self.expires_at
    }

    pub fn remaining_ttl(&self) -> Duration {
        self.expires_at
            .checked_duration_since(Instant::now())
            .unwrap_or_default()
    }
}

#[derive(Debug)]
pub struct DnsCache {
    entries: HashMap<String, DnsCacheEntry>,
    max_entries: usize,
    enabled: bool,
}

impl Default for DnsCache {
    fn default() -> Self {
        Self::new(1024)
    }
}

impl DnsCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_entries,
            enabled: true,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        if !enabled {
            self.clear();
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn get(
        &mut self,
        hostname: &str,
    ) -> Option<&DnsCacheEntry> {
        if !self.enabled {
            return None;
        }

        let expired = self
            .entries
            .get(hostname)
            .map(|entry| entry.expired())
            .unwrap_or(false);

        if expired {
            self.entries.remove(hostname);
            return None;
        }

        self.entries.get(hostname)
    }

    pub fn insert(
        &mut self,
        hostname: impl Into<String>,
        addresses: Vec<IpAddr>,
        ttl: Duration,
        secure: bool,
        validated: bool,
    ) {
        if !self.enabled || self.max_entries == 0 {
            return;
        }

        self.remove_expired();

        if self.entries.len() >= self.max_entries {
            self.evict_one();
        }

        let hostname = hostname.into();

        let entry = DnsCacheEntry::new(
            hostname.clone(),
            addresses,
            ttl,
            secure,
            validated,
        );

        self.entries.insert(hostname, entry);
    }

    pub fn remove(&mut self, hostname: &str) {
        self.entries.remove(hostname);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn remove_expired(&mut self) {
        self.entries.retain(|_, entry| !entry.expired());
    }

    fn evict_one(&mut self) {
        if let Some(key) = self
            .entries
            .iter()
            .min_by_key(|(_, entry)| entry.expires_at)
            .map(|(key, _)| key.clone())
        {
            self.entries.remove(&key);
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
