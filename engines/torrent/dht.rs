// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DhtState {
    Disabled,
    Starting,
    Running,
    Stopped,
}

pub struct DhtManager {
    state: DhtState,
    nodes: HashSet<String>,
}

impl DhtManager {
    pub fn new() -> Self {
        Self {
            state: DhtState::Disabled,
            nodes: HashSet::new(),
        }
    }

    pub fn enable(&mut self) {
        self.state = DhtState::Starting;
    }

    pub fn start(&mut self) {
        if self.state == DhtState::Starting {
            self.state = DhtState::Running;
        }
    }

    pub fn stop(&mut self) {
        self.state = DhtState::Stopped;
    }

    pub fn disable(&mut self) {
        self.state = DhtState::Disabled;
        self.nodes.clear();
    }

    pub fn add_node(&mut self, node: impl Into<String>) {
        if self.state == DhtState::Running {
            self.nodes.insert(node.into());
        }
    }

    pub fn remove_node(&mut self, node: &str) {
        self.nodes.remove(node);
    }

    pub fn state(&self) -> DhtState {
        self.state
    }

    pub fn nodes(&self) -> impl Iterator<Item = &String> {
        self.nodes.iter()
    }
}

impl Default for DhtManager {
    fn default() -> Self {
        Self::new()
    }
}
