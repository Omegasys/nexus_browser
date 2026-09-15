//! JavaScript engine loader.

use super::js_engine_api::{
    JsEngine,
    JsEngineError,
    JsEngineResult,
};
use super::registry::JsEngineRegistry;

/// Loads and initializes JavaScript engines.
pub struct JsEngineLoader;

impl JsEngineLoader {
    /// Creates a new loader.
    pub fn new() -> Self {
        Self
    }

    /// Loads an engine into the registry and optionally activates it.
    pub fn load_engine<E>(
        &self,
        registry: &mut JsEngineRegistry,
        engine: E,
        activate: bool,
    ) -> JsEngineResult<()>
    where
        E: JsEngine + 'static,
    {
        let name = engine.name().to_string();

        registry.register(engine)?;

        if activate {
            registry.set_active(&name)?;
            registry.initialize_active()?;
        }

        Ok(())
    }

    /// Starts an already loaded engine.
    pub fn start_active(
        &self,
        registry: &mut JsEngineRegistry,
    ) -> JsEngineResult<()> {
        let engine = registry.active_mut()?;

        engine.start()
    }

    /// Stops the active engine.
    pub fn stop_active(
        &self,
        registry: &mut JsEngineRegistry,
    ) -> JsEngineResult<()> {
        let engine = registry.active_mut()?;

        engine.stop()
    }

    /// Reloads the active engine.
    pub fn reload_active(
        &self,
        registry: &mut JsEngineRegistry,
    ) -> JsEngineResult<()> {
        let engine = registry.active_mut()?;

        if !engine.is_healthy() {
            return Err(JsEngineError::InvalidState);
        }

        engine.stop()?;
        engine.unload()?;
        engine.load()?;
        engine.start()?;

        Ok(())
    }
}

impl Default for JsEngineLoader {
    fn default() -> Self {
        Self::new()
    }
}
