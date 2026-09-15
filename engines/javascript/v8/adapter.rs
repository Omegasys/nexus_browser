//! V8 JavaScript engine adapter.

use super::process_launcher::V8ProcessLauncher;

use crate::browser_core::javascript::js_engine_api::{
    JsEngine,
    JsEngineCapabilities,
    JsEngineConfiguration,
    JsEngineError,
    JsEngineResult,
    JsEngineState,
};

/// Adapter around a V8 runtime.
pub struct V8Adapter {
    configuration: JsEngineConfiguration,
    capabilities: JsEngineCapabilities,
    state: JsEngineState,
    launcher: Option<V8ProcessLauncher>,
}

impl V8Adapter {
    /// Creates a V8 adapter using the default configuration.
    pub fn new() -> Self {
        Self::with_configuration(JsEngineConfiguration {
            name: "v8".to_string(),
            version: "12.0.0".to_string(),
            ..Default::default()
        })
    }

    /// Creates a V8 adapter with explicit configuration.
    pub fn with_configuration(configuration: JsEngineConfiguration) -> Self {
        Self {
            configuration,
            capabilities: JsEngineCapabilities::default(),
            state: JsEngineState::Unloaded,
            launcher: None,
        }
    }

    /// Returns the process launcher if one exists.
    pub fn launcher(&self) -> Option<&V8ProcessLauncher> {
        self.launcher.as_ref()
    }

    /// Creates a process launcher.
    fn create_launcher(&self) -> JsEngineResult<V8ProcessLauncher> {
        let executable = self
            .configuration
            .executable
            .as_deref()
            .ok_or_else(|| {
                JsEngineError::ConfigurationError(
                    "V8 executable path has not been configured".to_string(),
                )
            })?;

        V8ProcessLauncher::new(executable)
    }
}

impl Default for V8Adapter {
    fn default() -> Self {
        Self::new()
    }
}

impl JsEngine for V8Adapter {
    fn name(&self) -> &str {
        &self.configuration.name
    }

    fn version(&self) -> &str {
        &self.configuration.version
    }

    fn state(&self) -> JsEngineState {
        self.state
    }

    fn capabilities(&self) -> &JsEngineCapabilities {
        &self.capabilities
    }

    fn configuration(&self) -> &JsEngineConfiguration {
        &self.configuration
    }

    fn load(&mut self) -> JsEngineResult<()> {
        if self.state != JsEngineState::Unloaded {
            return Err(JsEngineError::AlreadyLoaded);
        }

        self.state = JsEngineState::Loaded;

        Ok(())
    }

    fn start(&mut self) -> JsEngineResult<()> {
        match self.state {
            JsEngineState::Loaded => {}
            JsEngineState::Running => {
                return Err(JsEngineError::AlreadyRunning);
            }
            JsEngineState::Unloaded => {
                return Err(JsEngineError::NotLoaded);
            }
            _ => {
                return Err(JsEngineError::InvalidState);
            }
        }

        self.state = JsEngineState::Starting;

        if self.configuration.executable.is_some() {
            let launcher = self.create_launcher()?;
            self.launcher = Some(launcher);
        }

        self.state = JsEngineState::Running;

        Ok(())
    }

    fn stop(&mut self) -> JsEngineResult<()> {
        if self.state != JsEngineState::Running {
            return Err(JsEngineError::NotRunning);
        }

        self.state = JsEngineState::Stopping;

        if let Some(launcher) = self.launcher.as_mut() {
            launcher.terminate()?;
        }

        self.state = JsEngineState::Loaded;

        Ok(())
    }

    fn unload(&mut self) -> JsEngineResult<()> {
        if self.state == JsEngineState::Running {
            return Err(JsEngineError::InvalidState);
        }

        self.launcher = None;
        self.state = JsEngineState::Unloaded;

        Ok(())
    }

    fn execute(&mut self, source: &str) -> JsEngineResult<String> {
        if self.state != JsEngineState::Running {
            return Err(JsEngineError::NotRunning);
        }

        if source.trim().is_empty() {
            return Ok(String::new());
        }

        Ok(format!("V8 execution requested: {} bytes", source.len()))
    }

    fn is_healthy(&self) -> bool {
        matches!(
            self.state,
            JsEngineState::Loaded | JsEngineState::Running
        )
    }
}
