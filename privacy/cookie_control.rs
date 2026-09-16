// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CookiePolicy {
    AllowAll,
    BlockThirdParty,
    BlockCrossSite,
    BlockAll,
}

#[derive(Debug, Clone)]
pub struct CookieRule {
    pub domain: String,
    pub allowed: bool,
}

pub struct CookieController {
    policy: CookiePolicy,
    rules: Vec<CookieRule>,
    session_only: bool,
}

impl CookieController {
    pub fn new() -> Self {
        Self {
            policy: CookiePolicy::BlockThirdParty,
            rules: Vec::new(),
            session_only: false,
        }
    }

    pub fn set_policy(&mut self, policy: CookiePolicy) {
        self.policy = policy;
    }

    pub fn set_third_party_blocking(&mut self, enabled: bool) {
        if enabled {
            self.policy = CookiePolicy::BlockThirdParty;
        } else {
            self.policy = CookiePolicy::AllowAll;
        }
    }

    pub fn set_session_only(&mut self, enabled: bool) {
        self.session_only = enabled;
    }

    pub fn add_rule(
        &mut self,
        domain: impl Into<String>,
        allowed: bool,
    ) {
        self.rules.push(CookieRule {
            domain: domain.into(),
            allowed,
        });
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    pub fn policy(&self) -> CookiePolicy {
        self.policy
    }

    pub fn session_only(&self) -> bool {
        self.session_only
    }

    pub fn rules(&self) -> &[CookieRule] {
        &self.rules
    }
}

impl Default for CookieController {
    fn default() -> Self {
        Self::new()
    }
}
