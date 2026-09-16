// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnssecStatus {
    Disabled,
    Valid,
    Invalid,
    Bogus,
    Insecure,
    Indeterminate,
}

pub struct DnssecValidator {
    enabled: bool,
    require_validation: bool,
}

impl DnssecValidator {
    pub fn new() -> Self {
        Self {
            enabled: true,
            require_validation: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn set_required(&mut self, required: bool) {
        self.require_validation = required;
    }

    pub fn validate(&self, status: DnssecStatus) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        match status {
            DnssecStatus::Valid => Ok(()),
            DnssecStatus::Insecure if !self.require_validation => Ok(()),
            DnssecStatus::Disabled => Ok(()),
            DnssecStatus::Invalid | DnssecStatus::Bogus => {
                Err("DNSSEC validation failed".into())
            }
            _ => Err("DNSSEC validation could not be established".into()),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_required(&self) -> bool {
        self.require_validation
    }
}

impl Default for DnssecValidator {
    fn default() -> Self {
        Self::new()
    }
}
