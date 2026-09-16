// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct CacheEntry {
    addresses: Vec<String>,
    expires_at: Instant,
}

pub struct DnsCache {
    entries: HashMap<String, CacheEntry>,
    default_ttl: Duration,
}

impl DnsCache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            default_ttl: Duration::from_secs(300),
        }
    }

    pub fn insert(
        &mut self,
        hostname: impl Into<String>,
        addresses: Vec<String>,
    ) {
        let hostname = hostname.into();

        self.entries.insert(
            hostname,
            CacheEntry {
                addresses,
                expires_at: Instant::now() + self.default_ttl,
            },
        );
    }

    pub fn get(&mut self, hostname: &str) -> Option<Vec<String>> {
        let expired = self
            .entries
            .get(hostname)
            .map(|entry| Instant::now() >= entry.expires_at)
            .unwrap_or(false);

        if expired {
            self.entries.remove(hostname);
            return None;
        }

        self.entries
            .get(hostname)
            .map(|entry| entry.addresses.clone())
    }

    pub fn remove(&mut self, hostname: &str) {
        self.entries.remove(hostname);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    pub fn set_default_ttl(&mut self, ttl: Duration) {
        self.default_ttl = ttl;
    }
}

impl Default for DnsCache {
    fn default() -> Self {
        Self::new()
    }
}
