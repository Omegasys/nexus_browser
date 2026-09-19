use super::cookie_manager::Cookie;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CookiePolicyMode {
    AllowAll,
    BlockThirdParty,
    BlockCrossSite,
    BlockAll,
    Custom,
}

#[derive(Debug, Clone)]
pub struct CookieContext {
    pub top_level_site: String,
    pub request_site: String,
    pub third_party: bool,
    pub cross_site: bool,
}

impl CookieContext {
    pub fn new(
        top_level_site: impl Into<String>,
        request_site: impl Into<String>,
    ) -> Self {
        let top_level_site = top_level_site.into();
        let request_site = request_site.into();

        let cross_site = top_level_site != request_site;

        Self {
            top_level_site,
            request_site,
            third_party: cross_site,
            cross_site,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CookiePolicy {
    pub mode: CookiePolicyMode,
    pub allow_secure: bool,
    pub allow_http_only: bool,
    pub require_partitioning: bool,
    pub block_tracking_cookies: bool,
}

impl Default for CookiePolicy {
    fn default() -> Self {
        Self::strict()
    }
}

impl CookiePolicy {
    pub fn allow_all() -> Self {
        Self {
            mode: CookiePolicyMode::AllowAll,
            allow_secure: false,
            allow_http_only: true,
            require_partitioning: false,
            block_tracking_cookies: false,
        }
    }

    pub fn strict() -> Self {
        Self {
            mode: CookiePolicyMode::BlockThirdParty,
            allow_secure: true,
            allow_http_only: true,
            require_partitioning: true,
            block_tracking_cookies: true,
        }
    }

    pub fn maximum() -> Self {
        Self {
            mode: CookiePolicyMode::BlockCrossSite,
            allow_secure: true,
            allow_http_only: true,
            require_partitioning: true,
            block_tracking_cookies: true,
        }
    }

    pub fn block_all() -> Self {
        Self {
            mode: CookiePolicyMode::BlockAll,
            allow_secure: true,
            allow_http_only: false,
            require_partitioning: true,
            block_tracking_cookies: true,
        }
    }

    pub fn allows_set(
        &self,
        cookie: &Cookie,
        context: &CookieContext,
    ) -> bool {
        if self.mode == CookiePolicyMode::BlockAll {
            return false;
        }

        if self.allow_secure && !cookie.secure {
            return false;
        }

        if self.require_partitioning
            && context.third_party
            && !cookie.partitioned
        {
            return false;
        }

        match self.mode {
            CookiePolicyMode::AllowAll => true,

            CookiePolicyMode::BlockThirdParty => {
                !context.third_party
            }

            CookiePolicyMode::BlockCrossSite => {
                !context.cross_site
            }

            CookiePolicyMode::BlockAll => false,

            CookiePolicyMode::Custom => true,
        }
    }

    pub fn allows_read(
        &self,
        _domain: &str,
        context: &CookieContext,
    ) -> bool {
        match self.mode {
            CookiePolicyMode::BlockAll => false,

            CookiePolicyMode::BlockThirdParty => {
                !context.third_party
            }

            CookiePolicyMode::BlockCrossSite => {
                !context.cross_site
            }

            _ => true,
        }
    }

    pub fn allows_send(
        &self,
        cookie: &Cookie,
        context: &CookieContext,
    ) -> bool {
        if self.mode == CookiePolicyMode::BlockAll {
            return false;
        }

        if context.third_party && self.require_partitioning {
            return cookie.partitioned;
        }

        true
    }
}
