// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferrerMode {
    Default,
    NoReferrer,
    Origin,
    StrictOrigin,
    SameOrigin,
}

pub struct ReferrerController {
    mode: ReferrerMode,
    strip_cross_origin: bool,
}

impl ReferrerController {
    pub fn new() -> Self {
        Self {
            mode: ReferrerMode::StrictOrigin,
            strip_cross_origin: true,
        }
    }

    pub fn set_mode(&mut self, mode: ReferrerMode) {
        self.mode = mode;
    }

    pub fn set_strip_cross_origin(&mut self, enabled: bool) {
        self.strip_cross_origin = enabled;
    }

    pub fn mode(&self) -> ReferrerMode {
        self.mode
    }

    pub fn should_strip_cross_origin(&self) -> bool {
        self.strip_cross_origin
    }

    pub fn generate(
        &self,
        source_origin: &str,
        target_origin: &str,
        full_url: &str,
    ) -> Option<String> {
        match self.mode {
            ReferrerMode::Default => Some(full_url.to_string()),

            ReferrerMode::NoReferrer => None,

            ReferrerMode::Origin => {
                Some(source_origin.to_string())
            }

            ReferrerMode::StrictOrigin => {
                if source_origin == target_origin {
                    Some(full_url.to_string())
                } else {
                    Some(source_origin.to_string())
                }
            }

            ReferrerMode::SameOrigin => {
                if source_origin == target_origin {
                    Some(full_url.to_string())
                } else {
                    None
                }
            }
        }
    }
}

impl Default for ReferrerController {
    fn default() -> Self {
        Self::new()
    }
}
