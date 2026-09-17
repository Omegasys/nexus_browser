use super::kill_switch::KillSwitch;
use super::network_enforcement::NetworkEnforcement;
use super::network_lock::NetworkLock;
use super::network_policy::NetworkPolicy;
use super::protocol_filter::ProtocolFilter;
use super::routing_manager::RoutingManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkManagerState {
    Disabled,
    Starting,
    Running,
    Locked,
    Error,
}

#[derive(Debug)]
pub struct NetworkManager {
    pub state: NetworkManagerState,
    pub routing: RoutingManager,
    pub policy: NetworkPolicy,
    pub enforcement: NetworkEnforcement,
    pub lock: NetworkLock,
    pub kill_switch: KillSwitch,
    pub protocol_filter: ProtocolFilter,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            state: NetworkManagerState::Disabled,
            routing: RoutingManager::new(),
            policy: NetworkPolicy::new(),
            enforcement: NetworkEnforcement::new(),
            lock: NetworkLock::new(),
            kill_switch: KillSwitch::new(),
            protocol_filter: ProtocolFilter::new(),
        }
    }

    pub fn start(&mut self) -> bool {
        if self.lock.is_locked() {
            self.state = NetworkManagerState::Locked;
            return false;
        }

        self.state = NetworkManagerState::Starting;

        self.kill_switch.enable();
        self.state = NetworkManagerState::Running;

        true
    }

    pub fn stop(&mut self) {
        self.state = NetworkManagerState::Disabled;
        self.kill_switch.enable();
    }

    pub fn lock(&mut self) {
        self.lock.lock();
        self.kill_switch.enable();
        self.state = NetworkManagerState::Locked;
    }

    pub fn unlock(&mut self) {
        self.lock.unlock();

        if self.state == NetworkManagerState::Locked {
            self.state = NetworkManagerState::Disabled;
        }
    }

    pub fn is_running(&self) -> bool {
        self.state == NetworkManagerState::Running
    }

    pub fn is_locked(&self) -> bool {
        self.lock.is_locked()
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for NetworkManager {
    fn default() -> Self {
        Self::new()
    }
}
