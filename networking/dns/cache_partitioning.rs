use std::collections::HashMap;

use super::dns_cache::{DnsCache, DnsCacheEntry};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DnsPartitionKey {
    pub top_level_site: String,
    pub network_context: String,
}

impl DnsPartitionKey {
    pub fn new(
        top_level_site: impl Into<String>,
        network_context: impl Into<String>,
    ) -> Self {
        Self {
            top_level_site: top_level_site.into(),
            network_context: network_context.into(),
        }
    }
}

#[derive(Debug)]
pub struct CachePartition {
    pub key: DnsPartitionKey,
    pub cache: DnsCache,
}

impl CachePartition {
    pub fn new(key: DnsPartitionKey, max_entries: usize) -> Self {
        Self {
            key,
            cache: DnsCache::new(max_entries),
        }
    }
}

#[derive(Debug)]
pub struct CachePartitionManager {
    partitions: HashMap<DnsPartitionKey, CachePartition>,
    max_entries_per_partition: usize,
}

impl Default for CachePartitionManager {
    fn default() -> Self {
        Self::new(256)
    }
}

impl CachePartitionManager {
    pub fn new(max_entries_per_partition: usize) -> Self {
        Self {
            partitions: HashMap::new(),
            max_entries_per_partition,
        }
    }

    pub fn get_or_create(
        &mut self,
        key: DnsPartitionKey,
    ) -> &mut CachePartition {
        self.partitions
            .entry(key.clone())
            .or_insert_with(|| {
                CachePartition::new(
                    key,
                    self.max_entries_per_partition,
                )
            })
    }

    pub fn get(
        &mut self,
        key: &DnsPartitionKey,
    ) -> Option<&mut CachePartition> {
        self.partitions.get_mut(key)
    }

    pub fn remove(&mut self, key: &DnsPartitionKey) {
        self.partitions.remove(key);
    }

    pub fn clear_partition(&mut self, key: &DnsPartitionKey) {
        if let Some(partition) = self.partitions.get_mut(key) {
            partition.cache.clear();
        }
    }

    pub fn clear_all(&mut self) {
        self.partitions.clear();
    }

    pub fn partition_count(&self) -> usize {
        self.partitions.len()
    }

    pub fn contains(&self, key: &DnsPartitionKey) -> bool {
        self.partitions.contains_key(key)
    }

    pub fn lookup(
        &mut self,
        key: &DnsPartitionKey,
        hostname: &str,
    ) -> Option<&DnsCacheEntry> {
        self.partitions
            .get_mut(key)
            .and_then(|partition| partition.cache.get(hostname))
    }
}
