// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsolationKey {
    pub first_party: String,
    pub origin: String,
}

impl IsolationKey {
    pub fn new(
        first_party: impl Into<String>,
        origin: impl Into<String>,
    ) -> Self {
        Self {
            first_party: first_party.into(),
            origin: origin.into(),
        }
    }
}

pub struct FirstPartyIsolation {
    enabled: bool,
    isolated_origins: HashMap<IsolationKey, String>,
}

impl FirstPartyIsolation {
    pub fn new() -> Self {
        Self {
            enabled: true,
            isolated_origins: HashMap::new(),
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn isolate(
        &mut self,
        key: IsolationKey,
        container_id: impl Into<String>,
    ) {
        if self.enabled {
            self.isolated_origins.insert(key, container_id.into());
        }
    }

    pub fn container_for(
        &self,
        key: &IsolationKey,
    ) -> Option<&String> {
        if !self.enabled {
            return None;
        }

        self.isolated_origins.get(key)
    }

    pub fn remove(&mut self, key: &IsolationKey) {
        self.isolated_origins.remove(key);
    }

    pub fn clear(&mut self) {
        self.isolated_origins.clear();
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for FirstPartyIsolation {
    fn default() -> Self {
        Self::new()
    }
}
