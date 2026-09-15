/*
 * Nexus Browser - Gecko Rendering Engine Adapter
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GeckoConfiguration {
    pub executable: PathBuf,
    pub library: Option<PathBuf>,
    pub javascript_enabled: bool,
    pub images_enabled: bool,
    pub webgl_enabled: bool,
    pub private_mode: bool,
    pub network_access: bool,
    pub filesystem_access: bool,
}

impl Default for GeckoConfiguration {
    fn default() -> Self {
        Self {
            executable: PathBuf::from("gecko"),
            library: None,
            javascript_enabled: true,
            images_enabled: true,
            webgl_enabled: false,
            private_mode: true,
            network_access: false,
            filesystem_access: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeckoState {
    Unloaded,
    Loading,
    Loaded,
    Running,
    Stopped,
    Failed,
}

pub struct GeckoAdapter {
    configuration: GeckoConfiguration,
    state: GeckoState,
}

impl GeckoAdapter {
    pub fn new(configuration: GeckoConfiguration) -> Self {
        Self {
            configuration,
            state: GeckoState::Unloaded,
        }
    }

    pub fn load(&mut self) -> Result<(), String> {
        if self.state == GeckoState::Loaded
            || self.state == GeckoState::Running
        {
            return Ok(());
        }

        self.state = GeckoState::Loading;

        if self.configuration.executable.as_os_str().is_empty() {
            self.state = GeckoState::Failed;
            return Err("Gecko executable path is empty".to_string());
        }

        self.state = GeckoState::Loaded;

        Ok(())
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.state == GeckoState::Unloaded {
            self.load()?;
        }

        if self.state != GeckoState::Loaded {
            return Err("Gecko engine is not loaded".to_string());
        }

        self.state = GeckoState::Running;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        if self.state == GeckoState::Running {
            self.state = GeckoState::Stopped;
        }

        Ok(())
    }

    pub fn unload(&mut self) -> Result<(), String> {
        self.state = GeckoState::Unloaded;
        Ok(())
    }

    pub fn state(&self) -> GeckoState {
        self.state
    }

    pub fn configuration(&self) -> &GeckoConfiguration {
        &self.configuration
    }

    pub fn is_running(&self) -> bool {
        self.state == GeckoState::Running
    }

    pub fn is_healthy(&self) -> bool {
        matches!(
            self.state,
            GeckoState::Loaded | GeckoState::Running
        )
    }
}
