/*
 * Nexus Browser - Servo Rendering Engine Adapter
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::path::PathBuf;

use crate::pipeline::RenderingPipeline;

#[derive(Debug, Clone)]
pub struct ServoConfiguration {
    pub executable: PathBuf,
    pub library: Option<PathBuf>,

    pub javascript_enabled: bool,
    pub images_enabled: bool,
    pub webgl_enabled: bool,

    pub private_mode: bool,
    pub network_access: bool,
    pub filesystem_access: bool,

    pub wasm_enabled: bool,
}

impl Default for ServoConfiguration {
    fn default() -> Self {
        Self {
            executable: PathBuf::from("servo"),
            library: None,

            javascript_enabled: true,
            images_enabled: true,
            webgl_enabled: false,

            private_mode: true,
            network_access: false,
            filesystem_access: false,

            wasm_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServoState {
    Unloaded,
    Loading,
    Loaded,
    Running,
    Stopped,
    Failed,
}

pub struct ServoAdapter {
    configuration: ServoConfiguration,
    state: ServoState,
    pipeline: Option<RenderingPipeline>,
}

impl ServoAdapter {
    pub fn new(configuration: ServoConfiguration) -> Self {
        Self {
            configuration,
            state: ServoState::Unloaded,
            pipeline: None,
        }
    }

    pub fn load(&mut self) -> Result<(), String> {
        if matches!(
            self.state,
            ServoState::Loaded | ServoState::Running
        ) {
            return Ok(());
        }

        self.state = ServoState::Loading;

        if self.configuration.executable.as_os_str().is_empty() {
            self.state = ServoState::Failed;

            return Err(
                "Servo executable path is empty".to_string()
            );
        }

        self.pipeline = Some(RenderingPipeline::new());

        self.state = ServoState::Loaded;

        Ok(())
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.state == ServoState::Unloaded {
            self.load()?;
        }

        if self.state != ServoState::Loaded {
            return Err(
                "Servo engine is not loaded".to_string()
            );
        }

        if let Some(pipeline) = self.pipeline.as_mut() {
            pipeline.initialize()?;
        }

        self.state = ServoState::Running;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        if let Some(pipeline) = self.pipeline.as_mut() {
            pipeline.shutdown()?;
        }

        if self.state == ServoState::Running {
            self.state = ServoState::Stopped;
        }

        Ok(())
    }

    pub fn unload(&mut self) -> Result<(), String> {
        self.stop()?;

        self.pipeline = None;
        self.state = ServoState::Unloaded;

        Ok(())
    }

    pub fn state(&self) -> ServoState {
        self.state
    }

    pub fn configuration(&self) -> &ServoConfiguration {
        &self.configuration
    }

    pub fn pipeline(&self) -> Option<&RenderingPipeline> {
        self.pipeline.as_ref()
    }

    pub fn pipeline_mut(&mut self) -> Option<&mut RenderingPipeline> {
        self.pipeline.as_mut()
    }

    pub fn is_running(&self) -> bool {
        self.state == ServoState::Running
    }

    pub fn is_healthy(&self) -> bool {
        matches!(
            self.state,
            ServoState::Loaded | ServoState::Running
        )
    }
}
