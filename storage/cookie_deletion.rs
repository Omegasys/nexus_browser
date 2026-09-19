use super::cookie_manager::CookieManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CookieDeletionScope {
    Cookie,
    Domain,
    Site,
    All,
}

#[derive(Debug, Clone)]
pub struct CookieDeletion {
    pub scope: CookieDeletionScope,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub name: Option<String>,
}

impl CookieDeletion {
    pub fn cookie(
        domain: impl Into<String>,
        path: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            scope: CookieDeletionScope::Cookie,
            domain: Some(domain.into()),
            path: Some(path.into()),
            name: Some(name.into()),
        }
    }

    pub fn domain(domain: impl Into<String>) -> Self {
        Self {
            scope: CookieDeletionScope::Domain,
            domain: Some(domain.into()),
            path: None,
            name: None,
        }
    }

    pub fn all() -> Self {
        Self {
            scope: CookieDeletionScope::All,
            domain: None,
            path: None,
            name: None,
        }
    }

    pub fn apply(&self, manager: &mut CookieManager) {
        match self.scope {
            CookieDeletionScope::Cookie => {
                if let (Some(domain), Some(path), Some(name)) =
                    (&self.domain, &self.path, &self.name)
                {
                    manager.remove_cookie(domain, path, name);
                }
            }

            CookieDeletionScope::Domain
            | CookieDeletionScope::Site => {
                if let Some(domain) = &self.domain {
                    manager.remove_domain(domain);
                }
            }

            CookieDeletionScope::All => {
                manager.clear();
            }
        }
    }
}
