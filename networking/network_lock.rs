#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkLockState {
    Unlocked,
    Locked,
}

#[derive(Debug, Clone)]
pub struct NetworkLock {
    state: NetworkLockState,
}

impl NetworkLock {
    pub fn new() -> Self {
        Self {
            state: NetworkLockState::Unlocked,
        }
    }

    pub fn lock(&mut self) {
        self.state = NetworkLockState::Locked;
    }

    pub fn unlock(&mut self) {
        self.state = NetworkLockState::Unlocked;
    }

    pub fn is_locked(&self) -> bool {
        self.state == NetworkLockState::Locked
    }

    pub fn state(&self) -> NetworkLockState {
        self.state
    }

    pub fn permits_network(&self) -> bool {
        !self.is_locked()
    }
}

impl Default for NetworkLock {
    fn default() -> Self {
        Self::new()
    }
}
