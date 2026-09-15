/*
 * Nexus Browser - Servo Process Launcher
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::path::Path;
use std::process::{Child, Command, Stdio};

pub struct ServoProcessLauncher {
    child: Option<Child>,
}

impl ServoProcessLauncher {
    pub fn new() -> Self {
        Self {
            child: None,
        }
    }

    pub fn launch(
        &mut self,
        executable: &Path,
    ) -> Result<(), String> {
        if self.child.is_some() {
            return Err(
                "Servo process is already running".to_string()
            );
        }

        let child = Command::new(executable)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| {
                format!("Failed to launch Servo: {error}")
            })?;

        self.child = Some(child);

        Ok(())
    }

    pub fn is_running(&mut self) -> bool {
        let Some(child) = self.child.as_mut() else {
            return false;
        };

        match child.try_wait() {
            Ok(Some(_)) => {
                self.child = None;
                false
            }

            Ok(None) => true,

            Err(_) => false,
        }
    }

    pub fn terminate(&mut self) -> Result<(), String> {
        if let Some(mut child) = self.child.take() {
            child.kill().map_err(|error| {
                format!(
                    "Failed to terminate Servo: {error}"
                )
            })?;
        }

        Ok(())
    }

    pub fn wait(&mut self) -> Result<(), String> {
        if let Some(mut child) = self.child.take() {
            child.wait().map_err(|error| {
                format!("Failed waiting for Servo: {error}")
            })?;
        }

        Ok(())
    }
}

impl Drop for ServoProcessLauncher {
    fn drop(&mut self) {
        let _ = self.terminate();
    }
}
