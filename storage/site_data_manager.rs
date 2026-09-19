use std::collections::HashSet;

use super::cache_manager::CacheManager;
use super::cookie_manager::CookieManager;
use super::indexeddb::IndexedDbManager;
use super::local_storage::LocalStorageManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SiteDataType {
    Cookies,
    Cache,
    IndexedDb,
    LocalStorage,
    SessionStorage,
    ServiceWorkers,
    All,
}

#[derive(Debug, Default)]
pub struct SiteDataManager {
    tracked_sites: HashSet<String>,
}

impl SiteDataManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_site(&mut self, site: impl Into<String>) {
        self.tracked_sites.insert(site.into());
    }

    pub fn unregister_site(&mut self, site: &str) {
        self.tracked_sites.remove(site);
    }

    pub fn sites(&self) -> impl Iterator<Item = &String> {
        self.tracked_sites.iter()
    }

    pub fn clear_site(
        &mut self,
        site: &str,
        cookies: &mut CookieManager,
        cache: &mut CacheManager,
        indexed_db: &mut IndexedDbManager,
        local_storage: &mut LocalStorageManager,
    ) {
        cookies.remove_domain(site);
        cache.clear_site(site);
        indexed_db.delete_site(site);
        local_storage.delete_site(site);

        self.unregister_site(site);
    }

    pub fn clear_all(
        &mut self,
        cookies: &mut CookieManager,
        cache: &mut CacheManager,
        indexed_db: &mut IndexedDbManager,
        local_storage: &mut LocalStorageManager,
    ) {
        cookies.clear();
        cache.clear();
        indexed_db.clear();
        local_storage.clear();

        self.tracked_sites.clear();
    }
}
