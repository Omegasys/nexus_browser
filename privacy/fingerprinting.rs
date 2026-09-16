// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FingerprintSurface {
    UserAgent,
    Screen,
    Window,
    Canvas,
    WebGl,
    Audio,
    Fonts,
    Timezone,
    Locale,
    HardwareConcurrency,
    DeviceMemory,
    MediaDevices,
    Sensors,
    WebRtc,
}

pub struct FingerprintProtection {
    enabled: bool,
    blocked_surfaces: HashSet<FingerprintSurface>,
    randomize: bool,
}

impl FingerprintProtection {
    pub fn new() -> Self {
        let mut blocked_surfaces = HashSet::new();

        blocked_surfaces.insert(FingerprintSurface::Canvas);
        blocked_surfaces.insert(FingerprintSurface::WebGl);
        blocked_surfaces.insert(FingerprintSurface::Audio);
        blocked_surfaces.insert(FingerprintSurface::Fonts);
        blocked_surfaces.insert(FingerprintSurface::MediaDevices);
        blocked_surfaces.insert(FingerprintSurface::Sensors);

        Self {
            enabled: true,
            blocked_surfaces,
            randomize: false,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_randomization(&mut self, enabled: bool) {
        self.randomize = enabled;
    }

    pub fn block(&mut self, surface: FingerprintSurface) {
        self.blocked_surfaces.insert(surface);
    }

    pub fn allow(&mut self, surface: &FingerprintSurface) {
        self.blocked_surfaces.remove(surface);
    }

    pub fn is_blocked(&self, surface: &FingerprintSurface) -> bool {
        self.enabled && self.blocked_surfaces.contains(surface)
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn randomization_enabled(&self) -> bool {
        self.randomize
    }

    pub fn blocked_surfaces(&self) -> impl Iterator<Item = &FingerprintSurface> {
        self.blocked_surfaces.iter()
    }
}

impl Default for FingerprintProtection {
    fn default() -> Self {
        Self::new()
    }
}
