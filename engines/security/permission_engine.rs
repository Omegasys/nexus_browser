//! Nexus Browser permission engine.
//!
//! Controls access between browser components,
//! engines, extensions, and system resources.

use std::collections::HashMap;



#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub enum Permission {


    FileSystem,

    Network,

    Camera,

    Microphone,

    Clipboard,

    Notifications,

    Location,

    DeviceAccess,

    ProcessCreation,

    KernelInterface,


}



#[derive(Debug, Clone)]

pub enum PermissionState {


    Allowed,

    Denied,

    Prompt,

    Restricted,

}



pub struct PermissionEngine {


    permissions:
        HashMap<String, HashMap<Permission, PermissionState>>,

}



impl PermissionEngine {


    pub fn new() -> Self {


        Self {

            permissions:
                HashMap::new(),

        }

    }



    pub fn register_component(
        &mut self,
        component: String
    ) {


        self.permissions
            .entry(component)
            .or_insert(
                HashMap::new()
            );

    }



    pub fn set_permission(
        &mut self,

        component: &str,

        permission: Permission,

        state: PermissionState,

    ) {


        if let Some(
            permissions
        ) =
            self.permissions
                .get_mut(component)
        {


            permissions.insert(
                permission,
                state
            );

        }

    }



    pub fn check_permission(
        &self,

        component: &str,

        permission: &Permission,

    ) -> PermissionState {


        self.permissions
            .get(component)

            .and_then(
                |p|
                p.get(permission)
            )

            .cloned()

            .unwrap_or(
                PermissionState::Restricted
            )

    }



    pub fn revoke_all(
        &mut self,
        component: &str
    ) {


        if let Some(
            permissions
        ) =
            self.permissions
                .get_mut(component)
        {


            for value in permissions.values_mut() {

                *value =
                    PermissionState::Denied;

            }

        }

    }

}


impl Default for PermissionEngine {


    fn default() -> Self {

        Self::new()

    }

}
