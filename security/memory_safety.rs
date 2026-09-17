use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryProtectionLevel {
    Standard,
    Strict,
    Maximum,
    Lockdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPermission {
    ReadOnly,
    ReadWrite,
    Execute,
    ReadExecute,
    NoAccess,
}

#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub id: u64,
    pub size: usize,
    pub permission: MemoryPermission,
    pub isolated: bool,
}

#[derive(Debug)]
pub struct MemoryProtection {
    pub level: MemoryProtectionLevel,
    regions: HashMap<u64, MemoryRegion>,
    next_region_id: u64,
}

impl MemoryProtection {
    pub fn new() -> Self {
        Self {
            level: MemoryProtectionLevel::Strict,
            regions: HashMap::new(),
            next_region_id: 1,
        }
    }

    pub fn set_level(&mut self, level: MemoryProtectionLevel) {
        self.level = level;
    }

    pub fn register_region(
        &mut self,
        size: usize,
        permission: MemoryPermission,
    ) -> u64 {
        let id = self.next_region_id;
        self.next_region_id += 1;

        let region = MemoryRegion {
            id,
            size,
            permission,
            isolated: matches!(
                self.level,
                MemoryProtectionLevel::Maximum
                    | MemoryProtectionLevel::Lockdown
            ),
        };

        self.regions.insert(id, region);
        id
    }

    pub fn remove_region(&mut self, id: u64) -> bool {
        self.regions.remove(&id).is_some()
    }

    pub fn get_region(&self, id: u64) -> Option<&MemoryRegion> {
        self.regions.get(&id)
    }

    pub fn set_permission(
        &mut self,
        id: u64,
        permission: MemoryPermission,
    ) -> bool {
        let Some(region) = self.regions.get_mut(&id) else {
            return false;
        };

        region.permission = permission;
        true
    }

    pub fn region_count(&self) -> usize {
        self.regions.len()
    }

    pub fn clear(&mut self) {
        self.regions.clear();
    }
}

impl Default for MemoryProtection {
    fn default() -> Self {
        Self::new()
    }
}
