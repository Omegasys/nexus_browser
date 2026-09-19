use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub key: String,
    pub site: String,
    pub content_type: String,
    pub data: Vec<u8>,
    pub expires_at: Option<Instant>,
    pub created_at: Instant,
}

impl CacheEntry {
    pub fn new(
        key: impl Into<String>,
        site: impl Into<String>,
        content_type: impl Into<String>,
        data: Vec<u8>,
        ttl: Option<Duration>,
    ) -> Self {
        let created_at = Instant::now();

        Self {
            key: key.into(),
            site: site.into(),
            content_type: content_type.into(),
            data,
            expires_at: ttl.map(|duration| created_at + duration),
            created_at,
        }
    }

    pub fn expired(&self) -> bool {
        self.expires_at
            .map(|expires| Instant::now() >= expires)
            .unwrap_or(false)
    }
}

#[derive(Debug)]
pub struct CacheManager {
    entries: HashMap<String, CacheEntry>,
    max_entries: usize,
    enabled: bool,
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new(4096)
    }
}

impl CacheManager {
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

    pub fn insert(&mut self, entry: CacheEntry) {
        if !self.enabled || self.max_entries == 0 {
            return;
        }

        self.remove_expired();

        if self.entries.len() >= self.max_entries {
            self.evict_one();
        }

        self.entries.insert(entry.key.clone(), entry);
    }

    pub fn get(&mut self, key: &str) -> Option<&CacheEntry> {
        if !self.enabled {
            return None;
        }

        let expired = self
            .entries
            .get(key)
            .map(|entry| entry.expired())
            .unwrap_or(false);

        if expired {
            self.entries.remove(key);
            return None;
        }

        self.entries.get(key)
    }

    pub fn remove(&mut self, key: &str) {
        self.entries.remove(key);
    }

    pub fn clear_site(&mut self, site: &str) {
        self.entries
            .retain(|_, entry| entry.site != site);
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
            .min_by_key(|(_, entry)| entry.created_at)
            .map(|(key, _)| key.clone())
        {
            self.entries.remove(&key);
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}
