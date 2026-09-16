// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelState {
    Unloaded,
    Loading,
    Ready,
    Running,
    Error,
}

#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub model_path: PathBuf,
    pub context_size: usize,
    pub gpu_acceleration: bool,
    pub threads: usize,
}

pub struct LocalModelRunner {
    state: ModelState,
    config: Option<ModelConfig>,
}

impl LocalModelRunner {
    pub fn new() -> Self {
        Self {
            state: ModelState::Unloaded,
            config: None,
        }
    }

    pub fn load(&mut self, config: ModelConfig) -> Result<(), String> {
        if config.context_size == 0 {
            return Err("Context size must be greater than zero".into());
        }

        if config.threads == 0 {
            return Err("Thread count must be greater than zero".into());
        }

        self.state = ModelState::Loading;
        self.config = Some(config);
        self.state = ModelState::Ready;

        Ok(())
    }

    pub fn unload(&mut self) {
        self.config = None;
        self.state = ModelState::Unloaded;
    }

    pub fn generate(&mut self, prompt: &str) -> Result<String, String> {
        if self.state != ModelState::Ready {
            return Err("Local model is not ready".into());
        }

        if prompt.is_empty() {
            return Err("Prompt cannot be empty".into());
        }

        self.state = ModelState::Running;

        // Placeholder for integration with a local inference runtime.
        let result = format!("Local model response placeholder: {}", prompt);

        self.state = ModelState::Ready;

        Ok(result)
    }

    pub fn state(&self) -> ModelState {
        self.state
    }

    pub fn config(&self) -> Option<&ModelConfig> {
        self.config.as_ref()
    }
}

impl Default for LocalModelRunner {
    fn default() -> Self {
        Self::new()
    }
}
