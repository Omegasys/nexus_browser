/*
 * Nexus Browser - Servo Renderer
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::pipeline::RenderingPipeline;

pub struct Renderer {
    pipeline: RenderingPipeline,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            pipeline: RenderingPipeline::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        self.pipeline.initialize()
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        self.pipeline.shutdown()
    }

    pub fn render_frame(&mut self) -> Result<(), String> {
        self.pipeline.render()
    }

    pub fn pipeline(&self) -> &RenderingPipeline {
        &self.pipeline
    }
}
