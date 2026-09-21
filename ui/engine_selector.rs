#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineCategory {
    Rendering,
    JavaScript,
    Networking,
    Dns,
    Storage,
    Graphics,
    Security,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineAvailability {
    Unknown,
    Available,
    Loading,
    Active,
    Failed,
    Quarantined,
}

#[derive(Debug, Clone)]
pub struct EngineOption {
    pub id: String,
    pub name: String,
    pub version: String,
    pub category: EngineCategory,
    pub availability: EngineAvailability,
    pub trusted: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct EngineSelector {
    engines: Vec<EngineOption>,
    selected: Vec<(EngineCategory, String)>,
    allow_runtime_switching: bool,
}

impl Default for EngineSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineSelector {
    pub fn new() -> Self {
        Self {
            engines: Vec::new(),
            selected: Vec::new(),
            allow_runtime_switching: true,
        }
    }

    pub fn register(&mut self, engine: EngineOption) {
        if let Some(existing) = self.engines.iter_mut().find(|item| item.id == engine.id) {
            *existing = engine;
        } else {
            self.engines.push(engine);
        }
    }

    pub fn remove(&mut self, id: &str) -> Option<EngineOption> {
        let position = self.engines.iter().position(|engine| engine.id == id)?;

        self.selected.retain(|(_, selected_id)| selected_id != id);

        Some(self.engines.remove(position))
    }

    pub fn engines(&self) -> &[EngineOption] {
        &self.engines
    }

    pub fn engines_for_category(
        &self,
        category: EngineCategory,
    ) -> Vec<&EngineOption> {
        self.engines
            .iter()
            .filter(|engine| engine.category == category)
            .collect()
    }

    pub fn select(
        &mut self,
        category: EngineCategory,
        engine_id: &str,
    ) -> Result<(), String> {
        let engine = self
            .engines
            .iter()
            .find(|engine| {
                engine.id == engine_id && engine.category == category
            })
            .ok_or_else(|| format!("engine '{engine_id}' was not found"))?;

        if !engine.enabled {
            return Err(format!("engine '{engine_id}' is disabled"));
        }

        if !matches!(
            engine.availability,
            EngineAvailability::Available | EngineAvailability::Active
        ) {
            return Err(format!(
                "engine '{engine_id}' is not available"
            ));
        }

        if !engine.trusted {
            return Err(format!(
                "engine '{engine_id}' is not trusted"
            ));
        }

        self.selected.retain(|(selected_category, _)| {
            *selected_category != category
        });

        self.selected
            .push((category, engine_id.to_string()));

        Ok(())
    }

    pub fn selected(
        &self,
        category: EngineCategory,
    ) -> Option<&EngineOption> {
        let id = self
            .selected
            .iter()
            .find(|(selected_category, _)| *selected_category == category)
            .map(|(_, id)| id)?;

        self.engines.iter().find(|engine| engine.id == id)
    }

    pub fn selected_id(
        &self,
        category: EngineCategory,
    ) -> Option<&str> {
        self.selected
            .iter()
            .find(|(selected_category, _)| *selected_category == category)
            .map(|(_, id)| id.as_str())
    }

    pub fn clear_selection(&mut self, category: EngineCategory) {
        self.selected
            .retain(|(selected_category, _)| *selected_category != category);
    }

    pub fn allow_runtime_switching(&self) -> bool {
        self.allow_runtime_switching
    }

    pub fn set_runtime_switching(&mut self, enabled: bool) {
        self.allow_runtime_switching = enabled;
    }

    pub fn can_switch_runtime(
        &self,
        category: EngineCategory,
    ) -> bool {
        if !self.allow_runtime_switching {
            return false;
        }

        self.engines
            .iter()
            .filter(|engine| engine.category == category)
            .filter(|engine| engine.enabled)
            .filter(|engine| engine.trusted)
            .filter(|engine| {
                matches!(
                    engine.availability,
                    EngineAvailability::Available
                        | EngineAvailability::Active
                )
            })
            .count()
            > 1
    }
}
