//! JavaScript engine registry.

use std::collections::HashMap;

use super::js_engine_api::{
    JsEngine,
    JsEngineError,
    JsEngineResult,
};

/// Registry containing available JavaScript engines.
pub struct JsEngineRegistry {
    engines: HashMap<String, Box<dyn JsEngine>>,
    active_engine: Option<String>,
}

impl JsEngineRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self {
            engines: HashMap::new(),
            active_engine: None,
        }
    }

    /// Registers a JavaScript engine.
    pub fn register<E>(&mut self, engine: E) -> JsEngineResult<()>
    where
        E: JsEngine + 'static,
    {
        let name = engine.name().to_string();

        if self.engines.contains_key(&name) {
            return Err(JsEngineError::EngineAlreadyRegistered(name));
        }

        self.engines.insert(name, Box::new(engine));

        Ok(())
    }

    /// Removes an engine from the registry.
    pub fn unregister(&mut self, name: &str) -> JsEngineResult<()> {
        if self.engines.remove(name).is_none() {
            return Err(JsEngineError::EngineNotFound(name.to_string()));
        }

        if self.active_engine.as_deref() == Some(name) {
            self.active_engine = None;
        }

        Ok(())
    }

    /// Returns whether an engine is registered.
    pub fn contains(&self, name: &str) -> bool {
        self.engines.contains_key(name)
    }

    /// Returns the number of registered engines.
    pub fn count(&self) -> usize {
        self.engines.len()
    }

    /// Lists registered engine names.
    pub fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.engines.keys().cloned().collect();
        names.sort();
        names
    }

    /// Selects the active engine.
    pub fn set_active(&mut self, name: &str) -> JsEngineResult<()> {
        if !self.engines.contains_key(name) {
            return Err(JsEngineError::EngineNotFound(name.to_string()));
        }

        self.active_engine = Some(name.to_string());

        Ok(())
    }

    /// Returns the active engine name.
    pub fn active_name(&self) -> Option<&str> {
        self.active_engine.as_deref()
    }

    /// Returns a mutable reference to the active engine.
    pub fn active_mut(&mut self) -> JsEngineResult<&mut dyn JsEngine> {
        let name = self
            .active_engine
            .as_deref()
            .ok_or_else(|| JsEngineError::EngineNotFound("no active engine".to_string()))?;

        self.engines
            .get_mut(name)
            .map(|engine| engine.as_mut())
            .ok_or_else(|| JsEngineError::EngineNotFound(name.to_string()))
    }

    /// Returns a mutable engine by name.
    pub fn get_mut(&mut self, name: &str) -> JsEngineResult<&mut dyn JsEngine> {
        self.engines
            .get_mut(name)
            .map(|engine| engine.as_mut())
            .ok_or_else(|| JsEngineError::EngineNotFound(name.to_string()))
    }

    /// Initializes the selected engine.
    pub fn initialize_active(&mut self) -> JsEngineResult<()> {
        let engine = self.active_mut()?;

        engine.load()?;
        engine.start()?;

        Ok(())
    }

    /// Shuts down the selected engine.
    pub fn shutdown_active(&mut self) -> JsEngineResult<()> {
        let engine = self.active_mut()?;

        engine.stop()?;
        engine.unload()?;

        Ok(())
    }
}

impl Default for JsEngineRegistry {
    fn default() -> Self {
        Self::new()
    }
}
