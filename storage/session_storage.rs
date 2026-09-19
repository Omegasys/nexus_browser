use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SessionStorage {
    site: String,
    values: HashMap<String, String>,
}

impl SessionStorage {
    pub fn new(site: impl Into<String>) -> Self {
        Self {
            site: site.into(),
            values: HashMap::new(),
        }
    }

    pub fn site(&self) -> &str {
        &self.site
    }

    pub fn set_item(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.values.insert(key.into(), value.into());
    }

    pub fn get_item(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    pub fn remove_item(&mut self, key: &str) -> Option<String> {
        self.values.remove(key)
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.values.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &String> {
        self.values.values()
    }

    pub fn entries(&self) -> impl Iterator<Item = (&String, &String)> {
        self.values.iter()
    }
}

#[derive(Debug, Default)]
pub struct SessionStorageManager {
    sessions: HashMap<String, SessionStorage>,
}

impl SessionStorageManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn get_or_create(&mut self, site: &str) -> &mut SessionStorage {
        self.sessions
            .entry(site.to_string())
            .or_insert_with(|| SessionStorage::new(site))
    }

    pub fn get(&self, site: &str) -> Option<&SessionStorage> {
        self.sessions.get(site)
    }

    pub fn get_mut(&mut self, site: &str) -> Option<&mut SessionStorage> {
        self.sessions.get_mut(site)
    }

    pub fn remove(&mut self, site: &str) -> Option<SessionStorage> {
        self.sessions.remove(site)
    }

    pub fn clear_site(&mut self, site: &str) {
        if let Some(storage) = self.sessions.get_mut(site) {
            storage.clear();
        }
    }

    pub fn clear_all(&mut self) {
        self.sessions.clear();
    }

    pub fn contains_site(&self, site: &str) -> bool {
        self.sessions.contains_key(site)
    }

    pub fn site_count(&self) -> usize {
        self.sessions.len()
    }
}
