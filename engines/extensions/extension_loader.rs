// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtensionState {
    Discovered,
    Loading,
    Loaded,
    Disabled,
    Blocked,
    Error,
}

#[derive(Debug, Clone)]
pub struct Extension {
    pub id: String,
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    pub state: ExtensionState,
}

pub struct ExtensionLoader {
    extensions: HashMap<String, Extension>,
}

impl ExtensionLoader {
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
        }
    }

    pub fn discover(
        &mut self,
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        path: PathBuf,
    ) -> Result<(), String> {
        let id = id.into();

        if self.extensions.contains_key(&id) {
            return Err("Extension already discovered".into());
        }

        self.extensions.insert(
            id.clone(),
            Extension {
                id,
                name: name.into(),
                version: version.into(),
                path,
                state: ExtensionState::Discovered,
            },
        );

        Ok(())
    }

    pub fn load(&mut self, id: &str) -> Result<(), String> {
        let extension = self
            .extensions
            .get_mut(id)
            .ok_or_else(|| "Extension not found".to_string())?;

        if extension.state == ExtensionState::Blocked {
            return Err("Extension is blocked".into());
        }

        extension.state = ExtensionState::Loading;

        // Placeholder for manifest validation and sandbox startup.

        extension.state = ExtensionState::Loaded;

        Ok(())
    }

    pub fn disable(&mut self, id: &str) {
        if let Some(extension) = self.extensions.get_mut(id) {
            extension.state = ExtensionState::Disabled;
        }
    }

    pub fn block(&mut self, id: &str) {
        if let Some(extension) = self.extensions.get_mut(id) {
            extension.state = ExtensionState::Blocked;
        }
    }

    pub fn get(&self, id: &str) -> Option<&Extension> {
        self.extensions.get(id)
    }

    pub fn extensions(&self) -> impl Iterator<Item = &Extension> {
        self.extensions.values()
    }
}

impl Default for ExtensionLoader {
    fn default() -> Self {
        Self::new()
    }
}
