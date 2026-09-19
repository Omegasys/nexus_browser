use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerState {
    Starting,
    Running,
    Stopped,
    Failed,
}

#[derive(Debug, Clone)]
pub struct SharedWorker {
    pub id: u64,
    pub site: String,
    pub name: String,
    pub script_url: String,
    pub state: WorkerState,
    pub created_at: SystemTime,
    connections: usize,
}

impl SharedWorker {
    pub fn new(
        id: u64,
        site: impl Into<String>,
        name: impl Into<String>,
        script_url: impl Into<String>,
    ) -> Self {
        Self {
            id,
            site: site.into(),
            name: name.into(),
            script_url: script_url.into(),
            state: WorkerState::Starting,
            created_at: SystemTime::now(),
            connections: 0,
        }
    }

    pub fn connect(&mut self) {
        self.connections += 1;

        if self.state == WorkerState::Starting {
            self.state = WorkerState::Running;
        }
    }

    pub fn disconnect(&mut self) {
        self.connections = self.connections.saturating_sub(1);

        if self.connections == 0 {
            self.state = WorkerState::Stopped;
        }
    }

    pub fn connection_count(&self) -> usize {
        self.connections
    }

    pub fn set_state(&mut self, state: WorkerState) {
        self.state = state;
    }
}

#[derive(Debug, Default)]
pub struct SharedWorkerManager {
    workers: HashMap<(String, String), SharedWorker>,
    next_id: u64,
}

impl SharedWorkerManager {
    pub fn new() -> Self {
        Self {
            workers: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn get_or_create(
        &mut self,
        site: &str,
        name: &str,
        script_url: &str,
    ) -> &mut SharedWorker {
        let key = (site.to_string(), name.to_string());

        if !self.workers.contains_key(&key) {
            let id = self.next_id;
            self.next_id += 1;

            self.workers.insert(
                key.clone(),
                SharedWorker::new(
                    id,
                    site,
                    name,
                    script_url,
                ),
            );
        }

        self.workers
            .get_mut(&key)
            .expect("shared worker must exist")
    }

    pub fn get(
        &self,
        site: &str,
        name: &str,
    ) -> Option<&SharedWorker> {
        self.workers
            .get(&(site.to_string(), name.to_string()))
    }

    pub fn get_mut(
        &mut self,
        site: &str,
        name: &str,
    ) -> Option<&mut SharedWorker> {
        self.workers
            .get_mut(&(site.to_string(), name.to_string()))
    }

    pub fn terminate(
        &mut self,
        site: &str,
        name: &str,
    ) -> bool {
        self.workers
            .remove(&(site.to_string(), name.to_string()))
            .is_some()
    }

    pub fn clear_site(&mut self, site: &str) {
        self.workers
            .retain(|(worker_site, _), _| worker_site != site);
    }

    pub fn clear_all(&mut self) {
        self.workers.clear();
    }

    pub fn count(&self) -> usize {
        self.workers.len()
    }
}
