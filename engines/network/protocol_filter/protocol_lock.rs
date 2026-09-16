//! Emergency protocol lock.
//!
//! When active, protocol traffic can be denied
//! until the lock is explicitly released.

pub struct ProtocolLock {
    locked: bool,
}

impl ProtocolLock {
    pub fn new() -> Self {
        Self {
            locked: false,
        }
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn unlock(&mut self) {
        self.locked = false;
    }

    pub fn active(&self) -> bool {
        self.locked
    }

    pub fn allowed(&self) -> bool {
        !self.locked
    }
}

impl Default for ProtocolLock {
    fn default() -> Self {
        Self::new()
    }
}
