// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExtensionPermission {
    Storage,
    Tabs,
    ActiveTab,
    Bookmarks,
    History,
    Cookies,
    Notifications,
    Clipboard,
    Downloads,
    WebRequest,
    Network,
    Scripting,
    FileSystem,
    NativeMessaging,
    Camera,
    Microphone,
    Location,
    Identity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionDecision {
    Denied,
    Prompt,
    Allowed,
}

pub struct PermissionModel {
    granted: HashSet<ExtensionPermission>,
    denied: HashSet<ExtensionPermission>,
}

impl PermissionModel {
    pub fn new() -> Self {
        Self {
            granted: HashSet::new(),
            denied: HashSet::new(),
        }
    }

    pub fn grant(&mut self, permission: ExtensionPermission) {
        self.denied.remove(&permission);
        self.granted.insert(permission);
    }

    pub fn deny(&mut self, permission: ExtensionPermission) {
        self.granted.remove(&permission);
        self.denied.insert(permission);
    }

    pub fn revoke(&mut self, permission: &ExtensionPermission) {
        self.granted.remove(permission);
        self.denied.remove(permission);
    }

    pub fn check(
        &self,
        permission: &ExtensionPermission,
    ) -> PermissionDecision {
        if self.granted.contains(permission) {
            PermissionDecision::Allowed
        } else if self.denied.contains(permission) {
            PermissionDecision::Denied
        } else {
            PermissionDecision::Prompt
        }
    }

    pub fn granted(&self) -> impl Iterator<Item = &ExtensionPermission> {
        self.granted.iter()
    }

    pub fn denied(&self) -> impl Iterator<Item = &ExtensionPermission> {
        self.denied.iter()
    }
}

impl Default for PermissionModel {
    fn default() -> Self {
        Self::new()
    }
}
