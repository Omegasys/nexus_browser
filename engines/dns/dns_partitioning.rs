// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DnsPartitionKey {
    pub profile_id: String,
    pub workspace_id: String,
    pub tab_id: String,
}

impl DnsPartitionKey {
    pub fn new(
        profile_id: impl Into<String>,
        workspace_id: impl Into<String>,
        tab_id: impl Into<String>,
    ) -> Self {
        Self {
            profile_id: profile_id.into(),
            workspace_id: workspace_id.into(),
            tab_id: tab_id.into(),
        }
    }
}

pub struct DnsPartitionManager {
    partitions: HashMap<DnsPartitionKey, HashMap<String, Vec<String>>>,
}

impl DnsPartitionManager {
    pub fn new() -> Self {
        Self {
            partitions: HashMap::new(),
        }
    }

    pub fn create_partition(&mut self, key: DnsPartitionKey) {
        self.partitions.entry(key).or_default();
    }

    pub fn remove_partition(&mut self, key: &DnsPartitionKey) {
        self.partitions.remove(key);
    }

    pub fn insert(
        &mut self,
        key: &DnsPartitionKey,
        hostname: impl Into<String>,
        addresses: Vec<String>,
    ) {
        self.partitions
            .entry(key.clone())
            .or_default()
            .insert(hostname.into(), addresses);
    }

    pub fn get(
        &self,
        key: &DnsPartitionKey,
        hostname: &str,
    ) -> Option<&Vec<String>> {
        self.partitions
            .get(key)
            .and_then(|partition| partition.get(hostname))
    }

    pub fn clear_partition(&mut self, key: &DnsPartitionKey) {
        if let Some(partition) = self.partitions.get_mut(key) {
            partition.clear();
        }
    }

    pub fn partition_count(&self) -> usize {
        self.partitions.len()
    }
}

impl Default for DnsPartitionManager {
    fn default() -> Self {
        Self::new()
    }
}
