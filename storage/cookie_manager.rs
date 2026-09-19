use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use super::cookie_policy::{CookieContext, CookiePolicy};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: SameSite,
    pub expires: Option<SystemTime>,
    pub max_age: Option<Duration>,
    pub partitioned: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl Cookie {
    pub fn new(
        name: impl Into<String>,
        value: impl Into<String>,
        domain: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            domain: domain.into(),
            path: "/".into(),
            secure: true,
            http_only: false,
            same_site: SameSite::Lax,
            expires: None,
            max_age: None,
            partitioned: false,
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires {
            if SystemTime::now() >= expires {
                return true;
            }
        }

        false
    }

    pub fn matches_domain(&self, domain: &str) -> bool {
        self.domain == domain
            || domain.ends_with(&format!(".{}", self.domain))
    }
}

#[derive(Debug, Default)]
pub struct CookieManager {
    cookies: HashMap<String, Vec<Cookie>>,
    policy: CookiePolicy,
}

impl CookieManager {
    pub fn new() -> Self {
        Self {
            cookies: HashMap::new(),
            policy: CookiePolicy::strict(),
        }
    }

    pub fn set_policy(&mut self, policy: CookiePolicy) {
        self.policy = policy;
    }

    pub fn policy(&self) -> &CookiePolicy {
        &self.policy
    }

    pub fn set_cookie(
        &mut self,
        cookie: Cookie,
        context: &CookieContext,
    ) -> Result<(), String> {
        if !self.policy.allows_set(&cookie, context) {
            return Err("Cookie rejected by policy".into());
        }

        if cookie.is_expired() {
            self.remove_cookie(
                &cookie.domain,
                &cookie.path,
                &cookie.name,
            );
            return Ok(());
        }

        let entry = self
            .cookies
            .entry(cookie.domain.clone())
            .or_default();

        entry.retain(|existing| {
            existing.name != cookie.name
                || existing.path != cookie.path
        });

        entry.push(cookie);

        Ok(())
    }

    pub fn get_cookies(
        &mut self,
        domain: &str,
        context: &CookieContext,
    ) -> Vec<Cookie> {
        self.remove_expired();

        if !self.policy.allows_read(domain, context) {
            return Vec::new();
        }

        self.cookies
            .get(domain)
            .into_iter()
            .flat_map(|cookies| cookies.iter())
            .filter(|cookie| cookie.matches_domain(domain))
            .filter(|cookie| self.policy.allows_send(cookie, context))
            .cloned()
            .collect()
    }

    pub fn remove_cookie(
        &mut self,
        domain: &str,
        path: &str,
        name: &str,
    ) {
        if let Some(cookies) = self.cookies.get_mut(domain) {
            cookies.retain(|cookie| {
                cookie.name != name || cookie.path != path
            });
        }
    }

    pub fn remove_domain(&mut self, domain: &str) {
        self.cookies.remove(domain);
    }

    pub fn remove_expired(&mut self) {
        self.cookies.retain(|_, cookies| {
            cookies.retain(|cookie| !cookie.is_expired());
            !cookies.is_empty()
        });
    }

    pub fn clear(&mut self) {
        self.cookies.clear();
    }

    pub fn domain_count(&self) -> usize {
        self.cookies.len()
    }
}
