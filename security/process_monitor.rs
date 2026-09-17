use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Starting,
    Running,
    Suspended,
    Terminating,
    Terminated,
    Suspicious,
    Quarantined,
}

#[derive(Debug, Clone)]
pub struct ProcessRecord {
    pub pid: u32,
    pub name: String,
    pub parent_pid: Option<u32>,
    pub state: ProcessState,
    pub started_at: SystemTime,
}

#[derive(Debug)]
pub struct ProcessMonitor {
    processes: HashMap<u32, ProcessRecord>,
}

impl ProcessMonitor {
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        pid: u32,
        name: impl Into<String>,
        parent_pid: Option<u32>,
    ) {
        self.processes.insert(
            pid,
            ProcessRecord {
                pid,
                name: name.into(),
                parent_pid,
                state: ProcessState::Starting,
                started_at: SystemTime::now(),
            },
        );
    }

    pub fn set_state(
        &mut self,
        pid: u32,
        state: ProcessState,
    ) -> bool {
        let Some(process) = self.processes.get_mut(&pid) else {
            return false;
        };

        process.state = state;
        true
    }

    pub fn mark_running(&mut self, pid: u32) -> bool {
        self.set_state(pid, ProcessState::Running)
    }

    pub fn mark_suspicious(&mut self, pid: u32) -> bool {
        self.set_state(pid, ProcessState::Suspicious)
    }

    pub fn mark_quarantined(&mut self, pid: u32) -> bool {
        self.set_state(pid, ProcessState::Quarantined)
    }

    pub fn mark_terminated(&mut self, pid: u32) -> bool {
        self.set_state(pid, ProcessState::Terminated)
    }

    pub fn get(&self, pid: u32) -> Option<&ProcessRecord> {
        self.processes.get(&pid)
    }

    pub fn is_running(&self, pid: u32) -> bool {
        self.get(pid)
            .map(|process| process.state == ProcessState::Running)
            .unwrap_or(false)
    }

    pub fn is_suspicious(&self, pid: u32) -> bool {
        self.get(pid)
            .map(|process| {
                matches!(
                    process.state,
                    ProcessState::Suspicious | ProcessState::Quarantined
                )
            })
            .unwrap_or(false)
    }

    pub fn remove(&mut self, pid: u32) -> bool {
        self.processes.remove(&pid).is_some()
    }

    pub fn processes(&self) -> impl Iterator<Item = &ProcessRecord> {
        self.processes.values()
    }

    pub fn clear(&mut self) {
        self.processes.clear();
    }
}

impl Default for ProcessMonitor {
    fn default() -> Self {
        Self::new()
    }
}
