/*
 * Nexus Browser - Servo WebAssembly Bridge
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmState {
    Disabled,
    Ready,
    Running,
    Failed,
}

pub struct WasmBridge {
    state: WasmState,
    modules: HashMap<String, Vec<u8>>,
}

impl WasmBridge {
    pub fn new() -> Self {
        Self {
            state: WasmState::Ready,
            modules: HashMap::new(),
        }
    }

    pub fn enable(&mut self) {
        self.state = WasmState::Ready;
    }

    pub fn disable(&mut self) {
        self.modules.clear();
        self.state = WasmState::Disabled;
    }

    pub fn register_module(
        &mut self,
        name: impl Into<String>,
        module: Vec<u8>,
    ) -> Result<(), String> {
        if self.state == WasmState::Disabled {
            return Err(
                "WebAssembly support is disabled".to_string()
            );
        }

        let name = name.into();

        if name.is_empty() {
            return Err(
                "WebAssembly module name is empty".to_string()
            );
        }

        if module.is_empty() {
            return Err(
                "WebAssembly module is empty".to_string()
            );
        }

        self.modules.insert(name, module);

        Ok(())
    }

    pub fn remove_module(
        &mut self,
        name: &str,
    ) -> bool {
        self.modules.remove(name).is_some()
    }

    pub fn has_module(
        &self,
        name: &str,
    ) -> bool {
        self.modules.contains_key(name)
    }

    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    pub fn state(&self) -> WasmState {
        self.state
    }

    pub fn is_enabled(&self) -> bool {
        self.state != WasmState::Disabled
    }

    pub fn start_module(
        &mut self,
        name: &str,
    ) -> Result<(), String> {
        if !self.is_enabled() {
            return Err(
                "WebAssembly support is disabled".to_string()
            );
        }

        if !self.has_module(name) {
            return Err(
                "WebAssembly module is not registered"
                    .to_string()
            );
        }

        self.state = WasmState::Running;

        Ok(())
    }
}
