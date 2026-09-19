use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceWorkerState {
    Installing,
    Installed,
    Activating,
    Activated,
    Redundant,
    Stopped,
}

#[derive(Debug, Clone)]
pub struct ServiceWorker {
    pub id: u64,
    pub site: String,
    pub scope: String,
    pub script_url: String,
    pub state: ServiceWorkerState,
    pub registered_at: SystemTime,
    pub last_updated: SystemTime,
}

impl ServiceWorker {
    pub fn new(
        id: u64,
        site: impl Into<String>,
        scope: impl Into<String>,
        script_url: impl Into<String>,
    ) -> Self {
        let now = SystemTime::now();

        Self {
            id,
            site: site.into(),
            scope: scope.into(),
            script_url: script_url.into(),
            state: ServiceWorkerState::Installing,
            registered_at: now,
            last_updated: now,
        }
    }

    pub fn set_state(&mut self, state: ServiceWorkerState) {
        self.state = state;
        self.last_updated = SystemTime::now();
    }

    pub fn controls_url(&self, url: &str) -> bool {
        url.starts_with(&self.scope)
    }
}

#[derive(Debug, Default)]
pub struct ServiceWorkerManager {
    workers: HashMap<u64, ServiceWorker>,
    next_id: u64,
}

impl ServiceWorkerManager {
    pub fn new() -> Self {
        Self {
            workers: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn register(
        &mut self,
        site: &str,
        scope: &str,
        script_url: &str,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let worker = ServiceWorker::new(
            id,
            site,
            scope,
            script_url,
        );

        self.workers.insert(id, worker);

        id
    }

    pub fn get(&self, id: u64) -> Option<&ServiceWorker> {
        self.workers.get(&id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut ServiceWorker> {
        self.workers.get_mut(&id)
    }

    pub fn unregister(&mut self, id: u64) -> bool {
        if let Some(worker) = self.workers.get_mut(&id) {
            worker.set_state(ServiceWorkerState::Redundant);
        }

        self.workers.remove(&id).is_some()
    }

    pub fn workers_for_site(&self, site: &str) -> Vec<&ServiceWorker> {
        self.workers
            .values()
            .filter(|worker| worker.site == site)
            .collect()
    }

    pub fn controller_for_url(
        &self,
        site: &str,
        url: &str,
    ) -> Option<&ServiceWorker> {
        self.workers
            .values()
            .filter(|worker| {
                worker.site == site
                    && worker.state == ServiceWorkerState::Activated
                    && worker.controls_url(url)
            })
            .max_by_key(|worker| worker.scope.len())
    }

    pub fn clear_site(&mut self, site: &str) {
        self.workers.retain(|_, worker| worker.site != site);
    }

    pub fn clear_all(&mut self) {
        self.workers.clear();
    }

    pub fn count(&self) -> usize {
        self.workers.len()
    }
}
