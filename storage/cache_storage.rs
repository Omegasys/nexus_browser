use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
pub struct CacheResponse {
    pub status: u16,
    pub content_type: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl CacheResponse {
    pub fn new(
        status: u16,
        content_type: impl Into<String>,
        body: Vec<u8>,
    ) -> Self {
        Self {
            status,
            content_type: content_type.into(),
            headers: HashMap::new(),
            body,
        }
    }

    pub fn set_header(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.headers.insert(name.into(), value.into());
    }
}

#[derive(Debug, Clone)]
pub struct CacheStorageEntry {
    pub request_url: String,
    pub response: CacheResponse,
    pub created_at: SystemTime,
    pub expires_at: Option<SystemTime>,
}

impl CacheStorageEntry {
    pub fn new(
        request_url: impl Into<String>,
        response: CacheResponse,
        ttl: Option<Duration>,
    ) -> Self {
        let created_at = SystemTime::now();

        let expires_at = ttl.map(|duration| {
            created_at
                .checked_add(duration)
                .unwrap_or(created_at)
        });

        Self {
            request_url: request_url.into(),
            response,
            created_at,
            expires_at,
        }
    }

    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expiration) => SystemTime::now() >= expiration,
            None => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheStorage {
    site: String,
    name: String,
    entries: HashMap<String, CacheStorageEntry>,
}

impl CacheStorage {
    pub fn new(
        site: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            site: site.into(),
            name: name.into(),
            entries: HashMap::new(),
        }
    }

    pub fn site(&self) -> &str {
        &self.site
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn put(
        &mut self,
        request_url: impl Into<String>,
        response: CacheResponse,
        ttl: Option<Duration>,
    ) {
        let url = request_url.into();

        self.entries.insert(
            url.clone(),
            CacheStorageEntry::new(url, response, ttl),
        );
    }

    pub fn match_request(
        &mut self,
        request_url: &str,
    ) -> Option<&CacheStorageEntry> {
        self.remove_expired();

        self.entries.get(request_url)
    }

    pub fn delete(&mut self, request_url: &str) -> bool {
        self.entries.remove(request_url).is_some()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&mut self) -> usize {
        self.remove_expired();
        self.entries.len()
    }

    pub fn is_empty(&mut self) -> bool {
        self.remove_expired();
        self.entries.is_empty()
    }

    pub fn keys(&mut self) -> Vec<String> {
        self.remove_expired();
        self.entries.keys().cloned().collect()
    }

    pub fn remove_expired(&mut self) {
        self.entries
            .retain(|_, entry| !entry.is_expired());
    }
}

#[derive(Debug, Default)]
pub struct CacheStorageManager {
    caches: HashMap<String, HashMap<String, CacheStorage>>,
}

impl CacheStorageManager {
    pub fn new() -> Self {
        Self {
            caches: HashMap::new(),
        }
    }

    pub fn open(
        &mut self,
        site: &str,
        cache_name: &str,
    ) -> &mut CacheStorage {
        self.caches
            .entry(site.to_string())
            .or_default()
            .entry(cache_name.to_string())
            .or_insert_with(|| CacheStorage::new(site, cache_name))
    }

    pub fn get(
        &self,
        site: &str,
        cache_name: &str,
    ) -> Option<&CacheStorage> {
        self.caches
            .get(site)
            .and_then(|site_caches| site_caches.get(cache_name))
    }

    pub fn get_mut(
        &mut self,
        site: &str,
        cache_name: &str,
    ) -> Option<&mut CacheStorage> {
        self.caches
            .get_mut(site)
            .and_then(|site_caches| site_caches.get_mut(cache_name))
    }

    pub fn delete(
        &mut self,
        site: &str,
        cache_name: &str,
    ) -> bool {
        let Some(site_caches) = self.caches.get_mut(site) else {
            return false;
        };

        let removed = site_caches.remove(cache_name).is_some();

        if site_caches.is_empty() {
            self.caches.remove(site);
        }

        removed
    }

    pub fn clear_site(&mut self, site: &str) {
        self.caches.remove(site);
    }

    pub fn clear_all(&mut self) {
        self.caches.clear();
    }

    pub fn cache_names(&self, site: &str) -> Vec<String> {
        self.caches
            .get(site)
            .map(|caches| caches.keys().cloned().collect())
            .unwrap_or_default()
    }
}
