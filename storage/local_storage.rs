use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LocalStorage {
    pub site: String,
    values: HashMap<String, String>,
}

impl LocalStorage {
    pub fn new(site: impl Into<String>) -> Self {
        Self {
            site: site.into(),
            values: HashMap::new(),
        }
    }

    pub fn set(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.values.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    pub fn remove(&mut self, key: &str) {
        self.values.remove(key);
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Debug, Default)]
pub struct LocalStorageManager {
    stores: HashMap<String, LocalStorage>,
}

impl LocalStorageManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_or_create(
        &mut self,
        site: impl Into<String>,
    ) -> &mut LocalStorage {
        let site = site.into();

        self.stores
            .entry(site.clone())
            .or_insert_with(|| LocalStorage::new(site))
    }

    pub fn get(&self, site: &str) -> Option<&LocalStorage> {
        self.stores.get(site)
    }

    pub fn get_mut(
        &mut self,
        site: &str,
    ) -> Option<&mut LocalStorage> {
        self.stores.get_mut(site)
    }

    pub fn delete_site(&mut self, site: &str) {
        self.stores.remove(site);
    }

    pub fn clear(&mut self) {
        self.stores.clear();
    }

    pub fn site_count(&self) -> usize {
        self.stores.len()
    }
}
