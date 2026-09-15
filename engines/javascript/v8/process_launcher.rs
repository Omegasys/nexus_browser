//! V8 process launcher.

use std::process::{
    Child,
    Command,
    Stdio,
};

use crate::browser_core::javascript::js_engine_api::{
    JsEngineError,
    JsEngineResult,
};

/// Launches an external V8 runtime process.
pub struct V8ProcessLauncher {
    executable: String,
    child: Option<Child>,
}

impl V8ProcessLauncher {
    /// Creates a new launcher.
    pub fn new(executable: impl Into<String>) -> JsEngineResult<Self> {
        let executable = executable.into();

        if executable.trim().is_empty() {
            return Err(JsEngineError::ConfigurationError(
                "V8 executable path cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            executable,
            child: None,
        })
    }

    /// Returns the configured executable path.
    pub fn executable(&self) -> &str {
        &self.executable
    }

    /// Launches the V8 process.
    pub fn launch(&mut self) -> JsEngineResult<()> {
        if self.child.is_some() {
            return Ok(());
        }

        let child = Command::new(&self.executable)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| {
                JsEngineError::LaunchFailed(error.to_string())
            })?;

        self.child = Some(child);

        Ok(())
    }

    /// Returns whether the V8 process is running.
    pub fn is_running(&mut self) -> bool {
        let Some(child) = self.child.as_mut() else {
            return false;
        };

        match child.try_wait() {
            Ok(Some(_)) => {
                self.child = None;
                false
            }
            Ok(None) => true,
            Err(_) => false,
        }
    }

    /// Terminates the V8 process.
    pub fn terminate(&mut self) -> JsEngineResult<()> {
        let Some(mut child) = self.child.take() else {
            return Ok(());
        };

        child.kill().map_err(|error| {
            JsEngineError::ShutdownFailed(error.to_string())
        })?;

        child.wait().map_err(|error| {
            JsEngineError::ShutdownFailed(error.to_string())
        })?;

        Ok(())
    }

    /// Waits for the V8 process to exit.
    pub fn wait(&mut self) -> JsEngineResult<()> {
        let Some(mut child) = self.child.take() else {
            return Ok(());
        };

        child.wait().map_err(|error| {
            JsEngineError::ShutdownFailed(error.to_string())
        })?;

        Ok(())
    }
}

impl Drop for V8ProcessLauncher {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
        }
    }
}
