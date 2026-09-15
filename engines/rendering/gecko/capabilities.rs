/*
 * Nexus Browser - Gecko Capabilities
 * Copyright (C) 2026
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    Render,
    JavaScript,
    Network,
    Tcp,
    Udp,
    Dns,
    Storage,
    TemporaryStorage,
    PersistentStorage,
    Cookies,
    WebRtc,
    WebGl,
    Gpu,
    Clipboard,
    Camera,
    Microphone,
    Notifications,
    FileRead,
    FileWrite,
    Downloads,
    Printing,
    DevTools,
}

#[derive(Debug, Clone)]
pub struct CapabilitySet {
    pub render: bool,
    pub javascript: bool,

    pub network: bool,
    pub tcp: bool,
    pub udp: bool,
    pub dns: bool,

    pub storage: bool,
    pub temporary_storage: bool,
    pub persistent_storage: bool,

    pub cookies: bool,
    pub webrtc: bool,

    pub webgl: bool,
    pub gpu: bool,

    pub clipboard: bool,
    pub camera: bool,
    pub microphone: bool,

    pub notifications: bool,

    pub file_read: bool,
    pub file_write: bool,

    pub downloads: bool,
    pub printing: bool,
    pub devtools: bool,
}

impl Default for CapabilitySet {
    fn default() -> Self {
        Self {
            render: true,
            javascript: true,

            network: false,
            tcp: false,
            udp: false,
            dns: false,

            storage: false,
            temporary_storage: true,
            persistent_storage: false,

            cookies: false,
            webrtc: false,

            webgl: false,
            gpu: false,

            clipboard: false,
            camera: false,
            microphone: false,

            notifications: false,

            file_read: false,
            file_write: false,

            downloads: false,
            printing: false,
            devtools: false,
        }
    }
}

impl CapabilitySet {
    pub fn has(&self, capability: Capability) -> bool {
        match capability {
            Capability::Render => self.render,
            Capability::JavaScript => self.javascript,
            Capability::Network => self.network,
            Capability::Tcp => self.tcp,
            Capability::Udp => self.udp,
            Capability::Dns => self.dns,
            Capability::Storage => self.storage,
            Capability::TemporaryStorage => self.temporary_storage,
            Capability::PersistentStorage => self.persistent_storage,
            Capability::Cookies => self.cookies,
            Capability::WebRtc => self.webrtc,
            Capability::WebGl => self.webgl,
            Capability::Gpu => self.gpu,
            Capability::Clipboard => self.clipboard,
            Capability::Camera => self.camera,
            Capability::Microphone => self.microphone,
            Capability::Notifications => self.notifications,
            Capability::FileRead => self.file_read,
            Capability::FileWrite => self.file_write,
            Capability::Downloads => self.downloads,
            Capability::Printing => self.printing,
            Capability::DevTools => self.devtools,
        }
    }

    pub fn revoke_all(&mut self) {
        *self = Self {
            render: true,
            temporary_storage: true,
            ..Self::default()
        };
    }
}
