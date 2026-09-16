// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutomationState {
    Disabled,
    Ready,
    Running,
    Paused,
    Error,
}

#[derive(Debug, Clone)]
pub struct AutomationTask {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub action: String,
}

pub struct AutomationEngine {
    state: AutomationState,
    tasks: HashMap<String, AutomationTask>,
}

impl AutomationEngine {
    pub fn new() -> Self {
        Self {
            state: AutomationState::Ready,
            tasks: HashMap::new(),
        }
    }

    pub fn enable(&mut self) {
        self.state = AutomationState::Ready;
    }

    pub fn disable(&mut self) {
        self.state = AutomationState::Disabled;
    }

    pub fn add_task(
        &mut self,
        id: impl Into<String>,
        name: impl Into<String>,
        action: impl Into<String>,
    ) -> Result<(), String> {
        let id = id.into();

        if self.tasks.contains_key(&id) {
            return Err("Automation task already exists".into());
        }

        self.tasks.insert(
            id.clone(),
            AutomationTask {
                id,
                name: name.into(),
                enabled: true,
                action: action.into(),
            },
        );

        Ok(())
    }

    pub fn remove_task(&mut self, id: &str) {
        self.tasks.remove(id);
    }

    pub fn enable_task(&mut self, id: &str) {
        if let Some(task) = self.tasks.get_mut(id) {
            task.enabled = true;
        }
    }

    pub fn disable_task(&mut self, id: &str) {
        if let Some(task) = self.tasks.get_mut(id) {
            task.enabled = false;
        }
    }

    pub fn run_task(&mut self, id: &str) -> Result<(), String> {
        if self.state == AutomationState::Disabled {
            return Err("Automation engine is disabled".into());
        }

        let task = self
            .tasks
            .get(id)
            .ok_or_else(|| "Automation task not found".to_string())?;

        if !task.enabled {
            return Err("Automation task is disabled".into());
        }

        self.state = AutomationState::Running;

        // Placeholder for the future browser automation runtime.

        self.state = AutomationState::Ready;

        Ok(())
    }

    pub fn tasks(&self) -> impl Iterator<Item = &AutomationTask> {
        self.tasks.values()
    }

    pub fn state(&self) -> AutomationState {
        self.state
    }
}

impl Default for AutomationEngine {
    fn default() -> Self {
        Self::new()
    }
}
