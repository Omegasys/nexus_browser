// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StoragePartitionKey {
    pub top_level_site: String,
    pub embedded_site: String,
}

impl StoragePartitionKey {
    pub fn new(
        top_level_site: impl Into<String>,
        embedded_site: impl Into<String>,
    ) -> Self {
        Self {
            top_level_site: top_level_site.into(),
            embedded_site: embedded_site.into(),
        }
    }
}

pub struct StatePartitioning {
    enabled: bool,
    partitions: HashMap<StoragePartitionKey, String>,
}

impl StatePartitioning {
    pub fn new() -> Self {
        Self {
            enabled: true,
            partitions: HashMap::new(),
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn get_partition(
        &self,
        key: &StoragePartitionKey,
    ) -> Option<&String> {
        if !self.enabled {
            return None;
        }

        self.partitions.get(key)
    }

    pub fn create_partition(
        &mut self,
        key: StoragePartitionKey,
        storage_id: impl Into<String>,
    ) {
        if self.enabled {
            self.partitions.insert(key, storage_id.into());
        }
    }

    pub fn remove_partition(&mut self, key: &StoragePartitionKey) {
        self.partitions.remove(key);
    }

    pub fn clear(&mut self) {
        self.partitions.clear();
    }

    pub fn partition_count(&self) -> usize {
        self.partitions.len()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for StatePartitioning {
    fn default() -> Self {
        Self::new()
    }
}
