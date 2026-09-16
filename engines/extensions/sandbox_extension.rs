// SPDX-License-Identifier: GPL-3.0-or-later

use super::permission_model::{
    ExtensionPermission,
    PermissionModel,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxState {
    Created,
    Running,
    Suspended,
    Terminated,
    Violated,
}

pub struct ExtensionSandbox {
    state: SandboxState,
    permissions: PermissionModel,
    network_access: bool,
    filesystem_access: bool,
    native_access: bool,
}

impl ExtensionSandbox {
    pub fn new() -> Self {
        Self {
            state: SandboxState::Created,
            permissions: PermissionModel::new(),
            network_access: false,
            filesystem_access: false,
            native_access: false,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.state == SandboxState::Terminated {
            return Err("Sandbox has been terminated".into());
        }

        self.state = SandboxState::Running;
        Ok(())
    }

    pub fn suspend(&mut self) {
        if self.state == SandboxState::Running {
            self.state = SandboxState::Suspended;
        }
    }

    pub fn terminate(&mut self) {
        self.state = SandboxState::Terminated;
    }

    pub fn grant_permission(&mut self, permission: ExtensionPermission) {
        match permission {
            ExtensionPermission::Network
            | ExtensionPermission::WebRequest => {
                self.network_access = true;
            }

            ExtensionPermission::FileSystem
            | ExtensionPermission::Downloads => {
                self.filesystem_access = true;
            }

            ExtensionPermission::NativeMessaging => {
                self.native_access = true;
            }

            _ => {}
        }

        self.permissions.grant(permission);
    }

    pub fn can_network(&self) -> bool {
        self.network_access
    }

    pub fn can_access_filesystem(&self) -> bool {
        self.filesystem_access
    }

    pub fn can_use_native_messaging(&self) -> bool {
        self.native_access
    }

    pub fn state(&self) -> SandboxState {
        self.state
    }

    pub fn permissions(&self) -> &PermissionModel {
        &self.permissions
    }
}

impl Default for ExtensionSandbox {
    fn default() -> Self {
        Self::new()
    }
}
