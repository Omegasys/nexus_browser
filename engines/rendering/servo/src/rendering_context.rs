/*
 * Nexus Browser - Servo Rendering Context
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::pipeline::{RenderingPipeline, Viewport};

pub struct RenderingContext {
    pipeline: RenderingPipeline,
    javascript_enabled: bool,
}

impl RenderingContext {
    pub fn new(javascript_enabled: bool) -> Self {
        Self {
            pipeline: RenderingPipeline::new(),
            javascript_enabled,
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        self.pipeline.initialize()
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        self.pipeline.shutdown()
    }

    pub fn render(&mut self) -> Result<(), String> {
        self.pipeline.render()
    }

    pub fn execute_javascript(
        &mut self,
        script: &str,
    ) -> Result<(), String> {
        if !self.javascript_enabled {
            return Err(
                "JavaScript is disabled".to_string()
            );
        }

        if !self.pipeline.is_running() {
            return Err(
                "Rendering pipeline is not running"
                    .to_string()
            );
        }

        if !script.is_empty() {
            self.pipeline.record_script_execution();
        }

        Ok(())
    }

    pub fn set_viewport(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        self.pipeline.set_viewport(width, height)
    }

    pub fn viewport(&self) -> Viewport {
        self.pipeline.viewport()
    }

    pub fn is_initialized(&self) -> bool {
        self.pipeline.is_running()
    }
}
