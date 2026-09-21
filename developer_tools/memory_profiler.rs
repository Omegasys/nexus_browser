use std::collections::HashMap;
use std::time::Instant;

/// Major memory categories used by the browser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryCategory {
    BrowserKernel,
    BrowserCore,
    Rendering,
    JavaScript,
    Network,
    Storage,
    Extensions,
    MicroVm,
    Graphics,
    Cache,
    Other,
}

/// A recorded allocation.
#[derive(Debug, Clone)]
pub struct MemoryAllocation {
    pub id: u64,
    pub category: MemoryCategory,
    pub size_bytes: u64,
    pub label: String,
    pub timestamp: Instant,
}

/// Point-in-time memory snapshot.
#[derive(Debug, Clone)]
pub struct MemorySnapshot {
    pub timestamp: Instant,
    pub total_bytes: u64,
    pub by_category: HashMap<MemoryCategory, u64>,
    pub allocation_count: usize,
}

/// Browser memory profiler.
#[derive(Debug, Clone)]
pub struct MemoryProfiler {
    allocations: HashMap<u64, MemoryAllocation>,
    snapshots: Vec<MemorySnapshot>,
    next_id: u64,
    enabled: bool,
}

impl MemoryProfiler {
    pub fn new() -> Self {
        Self {
            allocations: HashMap::new(),
            snapshots: Vec::new(),
            next_id: 1,
            enabled: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn record_allocation(
        &mut self,
        category: MemoryCategory,
        size_bytes: u64,
        label: impl Into<String>,
    ) -> Option<u64> {
        if !self.enabled {
            return None;
        }

        let id = self.next_id;
        self.next_id += 1;

        self.allocations.insert(
            id,
            MemoryAllocation {
                id,
                category,
                size_bytes,
                label: label.into(),
                timestamp: Instant::now(),
            },
        );

        Some(id)
    }

    pub fn release(&mut self, allocation_id: u64) -> bool {
        self.allocations.remove(&allocation_id).is_some()
    }

    pub fn allocation(
        &self,
        allocation_id: u64,
    ) -> Option<&MemoryAllocation> {
        self.allocations.get(&allocation_id)
    }

    pub fn allocations(&self) -> impl Iterator<Item = &MemoryAllocation> {
        self.allocations.values()
    }

    pub fn total_bytes(&self) -> u64 {
        self.allocations
            .values()
            .map(|allocation| allocation.size_bytes)
            .sum()
    }

    pub fn bytes_by_category(
        &self,
    ) -> HashMap<MemoryCategory, u64> {
        let mut result = HashMap::new();

        for allocation in self.allocations.values() {
            *result.entry(allocation.category).or_insert(0) +=
                allocation.size_bytes;
        }

        result
    }

    pub fn snapshot(&mut self) -> MemorySnapshot {
        let snapshot = MemorySnapshot {
            timestamp: Instant::now(),
            total_bytes: self.total_bytes(),
            by_category: self.bytes_by_category(),
            allocation_count: self.allocations.len(),
        };

        self.snapshots.push(snapshot.clone());

        snapshot
    }

    pub fn snapshots(&self) -> &[MemorySnapshot] {
        &self.snapshots
    }

    pub fn latest_snapshot(&self) -> Option<&MemorySnapshot> {
        self.snapshots.last()
    }

    pub fn clear_allocations(&mut self) {
        self.allocations.clear();
    }

    pub fn clear_snapshots(&mut self) {
        self.snapshots.clear();
    }

    pub fn clear(&mut self) {
        self.allocations.clear();
        self.snapshots.clear();
    }

    pub fn allocation_count(&self) -> usize {
        self.allocations.len()
    }

    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }
}

impl Default for MemoryProfiler {
    fn default() -> Self {
        Self::new()
    }
}
