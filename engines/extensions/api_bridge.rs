// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiPermission {
    ReadTabs,
    ModifyTabs,
    ReadStorage,
    WriteStorage,
    SendMessage,
    NetworkRequest,
    ExecuteScript,
}

#[derive(Debug, Clone)]
pub struct ApiRequest {
    pub extension_id: String,
    pub api: String,
    pub action: String,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ApiResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>,
}

pub struct ApiBridge {
    permissions: HashMap<String, Vec<ApiPermission>>,
}

impl ApiBridge {
    pub fn new() -> Self {
        Self {
            permissions: HashMap::new(),
        }
    }

    pub fn grant(
        &mut self,
        extension_id: impl Into<String>,
        permission: ApiPermission,
    ) {
        self.permissions
            .entry(extension_id.into())
            .or_default()
            .push(permission);
    }

    pub fn revoke(
        &mut self,
        extension_id: &str,
        permission: &ApiPermission,
    ) {
        if let Some(permissions) = self.permissions.get_mut(extension_id) {
            permissions.retain(|item| item != permission);
        }
    }

    pub fn check(
        &self,
        extension_id: &str,
        permission: &ApiPermission,
    ) -> bool {
        self.permissions
            .get(extension_id)
            .map(|permissions| permissions.contains(permission))
            .unwrap_or(false)
    }

    pub fn handle(
        &self,
        request: &ApiRequest,
        permission: ApiPermission,
    ) -> ApiResponse {
        if !self.check(&request.extension_id, &permission) {
            return ApiResponse {
                success: false,
                data: None,
                error: Some("Extension API permission denied".into()),
            };
        }

        ApiResponse {
            success: true,
            data: Some(format!(
                "API action '{}' executed",
                request.action
            )),
            error: None,
        }
    }
}

impl Default for ApiBridge {
    fn default() -> Self {
        Self::new()
    }
}
