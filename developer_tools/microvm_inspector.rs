use std::collections::HashMap;
use std::time::Instant;

/// MicroVM lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicroVmState {
    Creating,
    Starting,
    Running,
    Suspended,
    Snapshotting,
    Restoring,
    Recovering,
    Quarantined,
    Terminating,
    Terminated,
    Failed,
}

/// MicroVM subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MicroVmComponent {
    Renderer,
    JavaScript,
    Network,
    Dns,
    Storage,
    Gpu,
    Identity,
}

/// Information about a MicroVM.
#[derive(Debug, Clone)]
pub struct MicroVmInfo {
    pub id: String,
    pub workspace_id: Option<String>,
    pub tab_id: Option<String>,
    pub state: MicroVmState,
    pub components: Vec<MicroVmComponent>,
    pub memory_bytes: u64,
    pub cpu_percent: f64,
    pub snapshot_count: usize,
    pub escape_detection_enabled: bool,
    pub created_at: Instant,
    pub last_health_check: Instant,
}

/// MicroVM inspection summary.
#[derive(Debug, Clone)]
pub struct MicroVmInspectionResult {
    pub total_vms: usize,
    pub running_vms: usize,
    pub suspended_vms: usize,
    pub quarantined_vms: usize,
    pub failed_vms: usize,
    pub total_memory_bytes: u64,
    pub average_cpu_percent: f64,
}

/// Developer MicroVM inspector.
#[derive(Debug, Clone)]
pub struct MicroVmInspector {
    vms: HashMap<String, MicroVmInfo>,
}

impl MicroVmInspector {
    pub fn new() -> Self {
        Self {
            vms: HashMap::new(),
        }
    }

    pub fn register(&mut self, vm: MicroVmInfo) {
        self.vms.insert(vm.id.clone(), vm);
    }

    pub fn remove(&mut self, id: &str) -> Option<MicroVmInfo> {
        self.vms.remove(id)
    }

    pub fn get(&self, id: &str) -> Option<&MicroVmInfo> {
        self.vms.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut MicroVmInfo> {
        self.vms.get_mut(id)
    }

    pub fn update_state(
        &mut self,
        id: &str,
        state: MicroVmState,
    ) -> bool {
        if let Some(vm) = self.vms.get_mut(id) {
            vm.state = state;
            vm.last_health_check = Instant::now();
            true
        } else {
            false
        }
    }

    pub fn update_resources(
        &mut self,
        id: &str,
        memory_bytes: u64,
        cpu_percent: f64,
    ) -> bool {
        if let Some(vm) = self.vms.get_mut(id) {
            vm.memory_bytes = memory_bytes;
            vm.cpu_percent = cpu_percent.clamp(0.0, 100.0);
            vm.last_health_check = Instant::now();
            true
        } else {
            false
        }
    }

    pub fn vms(&self) -> impl Iterator<Item = &MicroVmInfo> {
        self.vms.values()
    }

    pub fn running_vms(&self) -> Vec<&MicroVmInfo> {
        self.vms
            .values()
            .filter(|vm| vm.state == MicroVmState::Running)
            .collect()
    }

    pub fn quarantined_vms(&self) -> Vec<&MicroVmInfo> {
        self.vms
            .values()
            .filter(|vm| vm.state == MicroVmState::Quarantined)
            .collect()
    }

    pub fn inspect(&self) -> MicroVmInspectionResult {
        let total = self.vms.len();

        let running = self
            .vms
            .values()
            .filter(|vm| vm.state == MicroVmState::Running)
            .count();

        let suspended = self
            .vms
            .values()
            .filter(|vm| vm.state == MicroVmState::Suspended)
            .count();

        let quarantined = self
            .vms
            .values()
            .filter(|vm| vm.state == MicroVmState::Quarantined)
            .count();

        let failed = self
            .vms
            .values()
            .filter(|vm| vm.state == MicroVmState::Failed)
            .count();

        let total_memory = self
            .vms
            .values()
            .map(|vm| vm.memory_bytes)
            .sum();

        let total_cpu: f64 =
            self.vms.values().map(|vm| vm.cpu_percent).sum();

        let average_cpu = if total == 0 {
            0.0
        } else {
            total_cpu / total as f64
        };

        MicroVmInspectionResult {
            total_vms: total,
            running_vms: running,
            suspended_vms: suspended,
            quarantined_vms: quarantined,
            failed_vms: failed,
            total_memory_bytes: total_memory,
            average_cpu_percent: average_cpu,
        }
    }

    pub fn clear(&mut self) {
        self.vms.clear();
    }
}

impl Default for MicroVmInspector {
    fn default() -> Self {
        Self::new()
    }
}
