use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct IndexedDbDatabase {
    pub site: String,
    pub name: String,
    pub version: u64,
    pub records: HashMap<String, Vec<u8>>,
}

impl IndexedDbDatabase {
    pub fn new(
        site: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            site: site.into(),
            name: name.into(),
            version: 1,
            records: HashMap::new(),
        }
    }

    pub fn put(
        &mut self,
        key: impl Into<String>,
        value: Vec<u8>,
    ) {
        self.records.insert(key.into(), value);
    }

    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        self.records.get(key)
    }

    pub fn delete(&mut self, key: &str) {
        self.records.remove(key);
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }
}

#[derive(Debug, Default)]
pub struct IndexedDbManager {
    databases: HashMap<String, IndexedDbDatabase>,
}

impl IndexedDbManager {
    pub fn new() -> Self {
        Self::default()
    }

    fn make_key(site: &str, name: &str) -> String {
        format!("{}::{}", site, name)
    }

    pub fn create_database(
        &mut self,
        site: impl Into<String>,
        name: impl Into<String>,
    ) {
        let site = site.into();
        let name = name.into();
        let key = Self::make_key(&site, &name);

        self.databases
            .entry(key)
            .or_insert_with(|| IndexedDbDatabase::new(site, name));
    }

    pub fn get(
        &self,
        site: &str,
        name: &str,
    ) -> Option<&IndexedDbDatabase> {
        self.databases.get(&Self::make_key(site, name))
    }

    pub fn get_mut(
        &mut self,
        site: &str,
        name: &str,
    ) -> Option<&mut IndexedDbDatabase> {
        self.databases.get_mut(&Self::make_key(site, name))
    }

    pub fn delete_database(
        &mut self,
        site: &str,
        name: &str,
    ) {
        self.databases.remove(&Self::make_key(site, name));
    }

    pub fn delete_site(&mut self, site: &str) {
        self.databases
            .retain(|_, database| database.site != site);
    }

    pub fn clear(&mut self) {
        self.databases.clear();
    }

    pub fn database_count(&self) -> usize {
        self.databases.len()
    }
}
