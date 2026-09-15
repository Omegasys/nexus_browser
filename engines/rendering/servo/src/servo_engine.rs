/*
 * Nexus Browser - Servo Engine
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::pipeline::RenderingPipeline;

pub struct ServoEngine {
    initialized: bool,
    running: bool,
    healthy: bool,

    pipeline: RenderingPipeline,

    pages_created: u64,
    frames_created: u64,
}

impl ServoEngine {
    pub fn new() -> Self {
        Self {
            initialized: false,
            running: false,
            healthy: false,

            pipeline: RenderingPipeline::new(),

            pages_created: 0,
            frames_created: 0,
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        if self.initialized {
            return Ok(());
        }

        self.pipeline.initialize()?;

        self.initialized = true;
        self.running = true;
        self.healthy = true;

        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        self.pipeline.shutdown()?;

        self.running = false;
        self.healthy = false;
        self.initialized = false;

        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), String> {
        self.shutdown()?;
        self.initialize()
    }

    pub fn create_page(&mut self) -> Result<u64, String> {
        if !self.running {
            return Err(
                "Servo engine is not running".to_string()
            );
        }

        self.pages_created += 1;

        Ok(self.pages_created)
    }

    pub fn create_frame(&mut self) -> Result<u64, String> {
        if !self.running {
            return Err(
                "Servo engine is not running".to_string()
            );
        }

        self.frames_created += 1;

        Ok(self.frames_created)
    }

    pub fn pipeline(&self) -> &RenderingPipeline {
        &self.pipeline
    }

    pub fn pipeline_mut(&mut self) -> &mut RenderingPipeline {
        &mut self.pipeline
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn is_healthy(&self) -> bool {
        self.healthy
    }
}
