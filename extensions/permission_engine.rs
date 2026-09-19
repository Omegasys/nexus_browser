use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    Tabs,
    Storage,
    Downloads,
    Bookmarks,
    History,
    Notifications,
    Clipboard,
    Network,
    Proxy,
    Dns,
    Cookies,
    FileSystem,
    NativeMessaging,
}

pub struct PermissionEngine {
    permissions: HashMap<String, HashSet<Permission>>,
}

impl PermissionEngine {
    pub fn new() -> Self {
        Self {
            permissions: HashMap::new(),
        }
    }

    pub fn grant(
        &mut self,
        extension_id: &str,
        permission: Permission,
    ) {
        self.permissions
            .entry(extension_id.to_string())
            .or_default()
            .insert(permission);
    }

    pub fn revoke(
        &mut self,
        extension_id: &str,
        permission: &Permission,
    ) {
        if let Some(set) =
            self.permissions.get_mut(extension_id)
        {
            set.remove(permission);
        }
    }

    pub fn has(
        &self,
        extension_id: &str,
        permission: &Permission,
    ) -> bool {
        self.permissions
            .get(extension_id)
            .map(|p| p.contains(permission))
            .unwrap_or(false)
    }
}
