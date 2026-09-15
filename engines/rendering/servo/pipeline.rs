/*
 * Nexus Browser - Servo Rendering Pipeline
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineState {
    Uninitialized,
    Initializing,
    Running,
    Stopping,
    Stopped,
    Failed,
}

#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PipelineMetrics {
    pub frames_rendered: u64,
    pub documents_loaded: u64,
    pub scripts_executed: u64,
}

pub struct RenderingPipeline {
    state: PipelineState,
    viewport: Viewport,
    metrics: PipelineMetrics,
}

impl RenderingPipeline {
    pub fn new() -> Self {
        Self {
            state: PipelineState::Uninitialized,
            viewport: Viewport::default(),
            metrics: PipelineMetrics::default(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        if self.state == PipelineState::Running {
            return Ok(());
        }

        self.state = PipelineState::Initializing;
        self.state = PipelineState::Running;

        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        if self.state == PipelineState::Stopped
            || self.state == PipelineState::Uninitialized
        {
            self.state = PipelineState::Stopped;
            return Ok(());
        }

        self.state = PipelineState::Stopping;
        self.state = PipelineState::Stopped;

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), String> {
        if self.state != PipelineState::Running {
            return Err(
                "Rendering pipeline is not running".to_string()
            );
        }

        self.metrics.frames_rendered += 1;

        Ok(())
    }

    pub fn set_viewport(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        if width == 0 || height == 0 {
            return Err(
                "Viewport dimensions must be non-zero"
                    .to_string()
            );
        }

        self.viewport = Viewport {
            width,
            height,
        };

        Ok(())
    }

    pub fn record_document_load(&mut self) {
        self.metrics.documents_loaded += 1;
    }

    pub fn record_script_execution(&mut self) {
        self.metrics.scripts_executed += 1;
    }

    pub fn state(&self) -> PipelineState {
        self.state
    }

    pub fn viewport(&self) -> Viewport {
        self.viewport
    }

    pub fn metrics(&self) -> &PipelineMetrics {
        &self.metrics
    }

    pub fn is_running(&self) -> bool {
        self.state == PipelineState::Running
    }
}
