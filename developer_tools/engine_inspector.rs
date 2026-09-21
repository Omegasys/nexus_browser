use std::collections::HashMap;
use std::time::Instant;

/// Engine category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EngineType {
    Rendering,
    JavaScript,
    Networking,
    Dns,
    Storage,
    Graphics,
    Security,
    Other,
}

/// Engine lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineState {
    Discovered,
    Installed,
    Staged,
    Testing,
    Limited,
    Approved,
    Trusted,
    Active,
    Suspended,
    Failed,
    Quarantined,
    RolledBack,
}

/// Engine capability.
#[derive(Debug, Clone)]
pub struct EngineCapability {
    pub name: String,
    pub enabled: bool,
    pub version: Option<String>,
}

/// Information about an engine.
#[derive(Debug, Clone)]
pub struct EngineInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub engine_type: EngineType,
    pub state: EngineState,
    pub capabilities: Vec<EngineCapability>,
    pub source_path: Option<String>,
    pub artifact_path: Option<String>,
    pub sandboxed: bool,
    pub hot_reload_supported: bool,
    pub last_health_check: Instant,
}

/// Engine inspection summary.
#[derive(Debug, Clone)]
pub struct EngineInspectionResult {
    pub total_engines: usize,
    pub active_engines: usize,
    pub trusted_engines: usize,
    pub failed_engines: usize,
    pub quarantined_engines: usize,
    pub by_type: HashMap<EngineType, usize>,
}

/// Developer engine inspector.
#[derive(Debug, Clone)]
pub struct EngineInspector {
    engines: HashMap<String, EngineInfo>,
}

impl EngineInspector {
    pub fn new() -> Self {
        Self {
            engines: HashMap::new(),
        }
    }

    pub fn register(&mut self, engine: EngineInfo) {
        self.engines.insert(engine.id.clone(), engine);
    }

    pub fn remove(&mut self, id: &str) -> Option<EngineInfo> {
        self.engines.remove(id)
    }

    pub fn get(&self, id: &str) -> Option<&EngineInfo> {
        self.engines.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut EngineInfo> {
        self.engines.get_mut(id)
    }

    pub fn update_state(
        &mut self,
        id: &str,
        state: EngineState,
    ) -> bool {
        if let Some(engine) = self.engines.get_mut(id) {
            engine.state = state;
            engine.last_health_check = Instant::now();
            true
        } else {
            false
        }
    }

    pub fn update_health_check(&mut self, id: &str) -> bool {
        if let Some(engine) = self.engines.get_mut(id) {
            engine.last_health_check = Instant::now();
            true
        } else {
            false
        }
    }

    pub fn engines(&self) -> impl Iterator<Item = &EngineInfo> {
        self.engines.values()
    }

    pub fn engines_by_type(
        &self,
        engine_type: EngineType,
    ) -> Vec<&EngineInfo> {
        self.engines
            .values()
            .filter(|engine| engine.engine_type == engine_type)
            .collect()
    }

    pub fn active_engine(
        &self,
        engine_type: EngineType,
    ) -> Option<&EngineInfo> {
        self.engines.values().find(|engine| {
            engine.engine_type == engine_type
                && engine.state == EngineState::Active
        })
    }

    pub fn inspect(&self) -> EngineInspectionResult {
        let mut active = 0;
        let mut trusted = 0;
        let mut failed = 0;
        let mut quarantined = 0;

        let mut by_type = HashMap::new();

        for engine in self.engines.values() {
            *by_type.entry(engine.engine_type).or_insert(0) += 1;

            match engine.state {
                EngineState::Active => active += 1,
                EngineState::Trusted => trusted += 1,
                EngineState::Failed => failed += 1,
                EngineState::Quarantined => quarantined += 1,
                _ => {}
            }
        }

        EngineInspectionResult {
            total_engines: self.engines.len(),
            active_engines: active,
            trusted_engines: trusted,
            failed_engines: failed,
            quarantined_engines: quarantined,
            by_type,
        }
    }

    pub fn failed_engines(&self) -> Vec<&EngineInfo> {
        self.engines
            .values()
            .filter(|engine| {
                matches!(
                    engine.state,
                    EngineState::Failed | EngineState::Quarantined
                )
            })
            .collect()
    }

    pub fn clear(&mut self) {
        self.engines.clear();
    }
}

impl Default for EngineInspector {
    fn default() -> Self {
        Self::new()
    }
}
