use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeState {
    Loaded,
    Running,
    Paused,
    Stopped,
    Crashed,
}

#[derive(Debug, Clone)]
pub struct ExtensionRuntime {
    pub extension_id: String,
    pub state: RuntimeState,
    pub memory_usage_mb: u64,
    pub cpu_percent: f32,
}

impl ExtensionRuntime {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            extension_id: id.into(),
            state: RuntimeState::Loaded,
            memory_usage_mb: 0,
            cpu_percent: 0.0,
        }
    }

    pub fn start(&mut self) {
        self.state = RuntimeState::Running;
    }

    pub fn pause(&mut self) {
        self.state = RuntimeState::Paused;
    }

    pub fn stop(&mut self) {
        self.state = RuntimeState::Stopped;
    }

    pub fn crash(&mut self) {
        self.state = RuntimeState::Crashed;
    }

    pub fn is_running(&self) -> bool {
        self.state == RuntimeState::Running
    }
}

pub struct RuntimeManager {
    runtimes: HashMap<String, ExtensionRuntime>,
}

impl RuntimeManager {
    pub fn new() -> Self {
        Self {
            runtimes: HashMap::new(),
        }
    }

    pub fn register(&mut self, runtime: ExtensionRuntime) {
        self.runtimes.insert(
            runtime.extension_id.clone(),
            runtime
        );
    }

    pub fn get(
        &self,
        id: &str
    ) -> Option<&ExtensionRuntime> {
        self.runtimes.get(id)
    }

    pub fn get_mut(
        &mut self,
        id: &str
    ) -> Option<&mut ExtensionRuntime> {
        self.runtimes.get_mut(id)
    }
}
