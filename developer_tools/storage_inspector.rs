use std::collections::HashMap;

/// Storage backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StorageType {
    Cookies,
    Cache,
    IndexedDb,
    LocalStorage,
    SessionStorage,
    CacheStorage,
    ServiceWorkers,
    SharedWorkers,
    Other,
}

/// Storage isolation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageIsolation {
    Global,
    Site,
    Origin,
    TopLevelSite,
    Workspace,
    Profile,
    Temporary,
    Disposable,
}

/// Information about storage belonging to a site.
#[derive(Debug, Clone)]
pub struct StorageEntry {
    pub site: String,
    pub storage_type: StorageType,
    pub isolation: StorageIsolation,
    pub entry_count: usize,
    pub size_bytes: u64,
    pub persistent: bool,
    pub encrypted: bool,
}

/// Storage inspection summary.
#[derive(Debug, Clone)]
pub struct StorageInspectionResult {
    pub total_entries: usize,
    pub total_bytes: u64,
    pub persistent_bytes: u64,
    pub encrypted_bytes: u64,
    pub by_type: HashMap<StorageType, u64>,
}

/// Developer storage inspector.
#[derive(Debug, Clone)]
pub struct StorageInspector {
    entries: Vec<StorageEntry>,
}

impl StorageInspector {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: StorageEntry) {
        self.entries.push(entry);
    }

    pub fn remove_site(&mut self, site: &str) {
        self.entries.retain(|entry| entry.site != site);
    }

    pub fn entries(&self) -> &[StorageEntry] {
        &self.entries
    }

    pub fn entries_for_site(
        &self,
        site: &str,
    ) -> Vec<&StorageEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.site == site)
            .collect()
    }

    pub fn inspect(&self) -> StorageInspectionResult {
        let total_entries = self
            .entries
            .iter()
            .map(|entry| entry.entry_count)
            .sum();

        let total_bytes = self
            .entries
            .iter()
            .map(|entry| entry.size_bytes)
            .sum();

        let persistent_bytes = self
            .entries
            .iter()
            .filter(|entry| entry.persistent)
            .map(|entry| entry.size_bytes)
            .sum();

        let encrypted_bytes = self
            .entries
            .iter()
            .filter(|entry| entry.encrypted)
            .map(|entry| entry.size_bytes)
            .sum();

        let mut by_type = HashMap::new();

        for entry in &self.entries {
            *by_type.entry(entry.storage_type).or_insert(0) +=
                entry.size_bytes;
        }

        StorageInspectionResult {
            total_entries,
            total_bytes,
            persistent_bytes,
            encrypted_bytes,
            by_type,
        }
    }

    pub fn total_bytes(&self) -> u64 {
        self.entries
            .iter()
            .map(|entry| entry.size_bytes)
            .sum()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for StorageInspector {
    fn default() -> Self {
        Self::new()
    }
}
